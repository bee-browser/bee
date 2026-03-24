use std::alloc::Layout;
use std::collections::VecDeque;
use std::ptr::NonNull;

use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

use crate::handle::Handle;
use crate::handle::HandleMut;
use crate::handle::Seq;

/// A heap memory managed by GC.
pub struct Heap {
    /// A set of addresses of *managed* memory blocks.
    memories: FxHashMap<usize, Memory>,

    /// A set of tracing targets.
    ///
    /// The set MAY contain addresses of *unmanaged* memory blocks.
    tracees: FxHashMap<usize, Tracee>,
}

impl Heap {
    /// Creates a heap.
    pub fn new() -> Self {
        Self {
            memories: Default::default(),
            tracees: Default::default(),
        }
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc<T>(&mut self, object: T) -> Handle<T>
    where
        T: Sized + Unknown,
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            assert!(!ptr.is_null());
            std::ptr::write(ptr, object);
            ptr
        };

        self.memories.insert(
            ptr as usize,
            Memory {
                layout: Layout::new::<T>(),
            },
        );
        self.tracees.insert(
            ptr as usize,
            Tracee {
                vtable: T::vtable(),
            },
        );

        Handle::from_ptr(ptr).unwrap()
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc_mut<T>(&mut self, object: T) -> HandleMut<T>
    where
        T: Sized + Unknown,
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            assert!(!ptr.is_null());
            std::ptr::write(ptr, object);
            ptr
        };

        self.memories.insert(
            ptr as usize,
            Memory {
                layout: Layout::new::<T>(),
            },
        );
        self.tracees.insert(
            ptr as usize,
            Tracee {
                vtable: T::vtable(),
            },
        );

        HandleMut::from_ptr(ptr).unwrap()
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc_layout<T, F>(&mut self, layout: Layout, init: F) -> Handle<T>
    where
        T: Sized + Unknown,
        F: FnOnce(NonNull<u8>),
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            NonNull::new(std::alloc::alloc(layout)).unwrap()
        };

        init(ptr);

        self.memories.insert(ptr.addr().get(), Memory { layout });
        self.tracees.insert(
            ptr.addr().get(),
            Tracee {
                vtable: T::vtable(),
            },
        );

        Handle::from_ref(unsafe { ptr.cast::<T>().as_ref() })
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc_layout_mut<T, F>(&mut self, layout: Layout, init: F) -> HandleMut<T>
    where
        T: Sized + Unknown,
        F: FnOnce(NonNull<u8>),
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            NonNull::new(std::alloc::alloc(layout)).unwrap()
        };

        init(ptr);

        self.memories.insert(ptr.addr().get(), Memory { layout });
        self.tracees.insert(
            ptr.addr().get(),
            Tracee {
                vtable: T::vtable(),
            },
        );

        HandleMut::from_mut(unsafe { ptr.cast::<T>().as_mut() })
    }

    // TODO: return HandleMut
    // TODO: there is no way to restrict the type of `T` to an integer type.
    pub fn alloc_seq<T>(&mut self, src: &[T]) -> Seq<T>
    where
        T: Atom,
    {
        let len = src.len();

        let layout = Layout::array::<T>(len).unwrap();

        let data = unsafe {
            // TODO(perf): use a dedicated memory pool
            let ptr = NonNull::new(std::alloc::alloc(layout)).unwrap();
            ptr.cast::<T>().as_ptr().copy_from(src.as_ptr(), len);
            Handle::from_ref(ptr.cast::<T>().as_ref())
        };

        self.memories.insert(data.as_addr(), Memory { layout });
        // No need to trace.

        Seq { data, len }
    }

    pub fn alloc_seq_with_init<T, F>(&mut self, len: usize, init: F) -> Seq<T>
    where
        T: Atom,
        F: FnOnce(NonNull<T>),
    {
        let layout = Layout::array::<T>(len).unwrap();

        let (ptr, data) = unsafe {
            // TODO(perf): use a dedicated memory pool
            let ptr = NonNull::new(std::alloc::alloc(layout)).unwrap();
            (ptr.cast::<T>(), Handle::from_ref(ptr.cast::<T>().as_ref()))
        };

        init(ptr);

        self.memories.insert(data.as_addr(), Memory { layout });
        // No need to trace.

        Seq { data, len }
    }

    // TODO(feat): not ergonomic... need a way to prevent UAF.
    pub fn add_tracee<T>(&mut self, target: Handle<T>)
    where
        T: Unknown,
    {
        let addr = target.as_addr();
        debug_assert!(!self.tracees.contains_key(&addr));
        self.tracees.insert(
            addr,
            Tracee {
                vtable: T::vtable(),
            },
        );
    }

    // TODO(feat): not ergonomic... need a way to prevent UAF.
    pub fn remove_tracee<T>(&mut self, target: Handle<T>)
    where
        T: Unknown,
    {
        let addr = target.as_addr();
        debug_assert!(self.tracees.contains_key(&addr));
        self.tracees.remove(&addr);
    }

    /// Reclaims objects that are not reachable from a specified root objects.
    pub fn collect_garbage(&mut self, roots: &[usize]) {
        let mut state = GcState::new(roots);
        self.mark(&mut state);
        self.sweep(&mut state);
    }

    /// Performs the mark phase.
    fn mark(&mut self, state: &mut GcState) {
        while let Some(addr) = state.visit_list.pop() {
            if state.visited.contains(&addr) {
                continue;
            }
            state.visited.insert(addr);
            if let Some(tracee) = self.tracees.get(&addr) {
                if let Some(trace) = tracee.vtable.trace {
                    trace(addr, &mut state.visit_list);
                }
            }
        }
    }

    /// Performs the sweep phase.
    fn sweep(&mut self, state: &mut GcState) {
        for (addr, memory) in self
            .memories
            .extract_if(|addr, _| !state.visited.contains(addr))
        {
            if let Some(tracee) = self.tracees.remove(&addr) {
                if let Some(tidy) = tracee.vtable.tidy {
                    tidy(addr);
                }
            }
            // SAFETY: the memory block was allocated by using std::alloc::alloc().
            unsafe {
                std::alloc::dealloc(addr as *mut u8, memory.layout);
            }
        }
    }

    /// Returns statistics.
    pub fn stats(&self) -> Stats {
        Stats {
            num_objects: self.memories.len(),
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics of a heap at some point.
pub struct Stats {
    pub num_objects: usize,
}

/// A data type holds GC states.
struct GcState {
    visit_list: VisitList,
    visited: FxHashSet<usize>,
}

impl GcState {
    fn new(roots: &[usize]) -> Self {
        let mut visit_list: VisitList = Default::default();
        visit_list.extend(roots.iter().cloned());
        Self {
            visit_list,
            visited: Default::default(),
        }
    }
}

/// A list to which reachable objects will be added.
#[derive(Default)]
pub struct VisitList(VecDeque<usize>);

impl VisitList {
    /// Appends a handle to the back of the visit list.
    pub fn push(&mut self, addr: usize) {
        self.0.push_back(addr);
    }

    /// Appends handles of an iterator.
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = usize>,
    {
        self.0.extend(iter);
    }

    /// Removes the first handle and returns it, or `None` if the visit list is empty.
    fn pop(&mut self) -> Option<usize> {
        self.0.pop_front()
    }
}

struct Memory {
    layout: Layout,
}

struct Tracee {
    vtable: &'static UnknownVtable,
}

pub struct UnknownVtable {
    pub tidy: Option<TidyFn>,
    pub trace: Option<TraceFn>,
}

type TidyFn = fn(usize);
type TraceFn = fn(usize, &mut VisitList);

pub trait Unknown {
    fn vtable() -> &'static UnknownVtable;
}

pub trait Atom: Copy + Sized {}
impl Atom for u16 {}
