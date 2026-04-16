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
    memory_blocks: FxHashMap<usize, MemoryBlock>,

    /// A set of tracing targets.
    ///
    /// The set MAY contain addresses of *unmanaged* memory blocks.
    trace_targets: FxHashMap<usize, Tracer>,
}

impl Heap {
    /// Creates a heap.
    pub fn new() -> Self {
        Self {
            memory_blocks: Default::default(),
            trace_targets: Default::default(),
        }
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc<T>(&mut self, object: T) -> Handle<T>
    where
        T: Sized + Trace,
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            assert!(!ptr.is_null());
            std::ptr::write(ptr, object);
            ptr
        };

        self.memory_blocks
            .insert(ptr as usize, MemoryBlock::new::<T>());
        self.trace_targets.insert(ptr as usize, Tracer::new::<T>());

        Handle::from_ptr(ptr).unwrap()
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc_mut<T>(&mut self, object: T) -> HandleMut<T>
    where
        T: Sized + Trace,
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            assert!(!ptr.is_null());
            std::ptr::write(ptr, object);
            ptr
        };

        self.memory_blocks
            .insert(ptr as usize, MemoryBlock::new::<T>());
        self.trace_targets.insert(ptr as usize, Tracer::new::<T>());

        HandleMut::from_ptr(ptr).unwrap()
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc_layout<T, F>(&mut self, layout: Layout, init: F) -> Handle<T>
    where
        T: Sized + Trace,
        F: FnOnce(NonNull<u8>),
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            NonNull::new(std::alloc::alloc(layout)).unwrap()
        };

        init(ptr);

        self.memory_blocks
            .insert(ptr.addr().get(), MemoryBlock::with_layout::<T>(layout));
        self.trace_targets
            .insert(ptr.addr().get(), Tracer::new::<T>());

        Handle::from_ref(unsafe { ptr.cast::<T>().as_ref() })
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc_layout_mut<T, F>(&mut self, layout: Layout, init: F) -> HandleMut<T>
    where
        T: Sized + Trace,
        F: FnOnce(NonNull<u8>),
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            NonNull::new(std::alloc::alloc(layout)).unwrap()
        };

        init(ptr);

        self.memory_blocks
            .insert(ptr.addr().get(), MemoryBlock::with_layout::<T>(layout));
        self.trace_targets
            .insert(ptr.addr().get(), Tracer::new::<T>());

        HandleMut::from_mut(unsafe { ptr.cast::<T>().as_mut() })
    }

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

        self.memory_blocks
            .insert(data.as_addr(), MemoryBlock::with_layout::<T>(layout));
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

        self.memory_blocks
            .insert(data.as_addr(), MemoryBlock::with_layout::<T>(layout));
        // No need to trace.

        Seq { data, len }
    }

    // TODO(feat): not ergonomic... need a way to prevent UAF.
    pub fn add_tracer<T>(&mut self, target: Handle<T>)
    where
        T: Trace,
    {
        let addr = target.as_addr();
        debug_assert!(!self.trace_targets.contains_key(&addr));
        self.trace_targets.insert(addr, Tracer::new::<T>());
    }

    // TODO(feat): not ergonomic... need a way to prevent UAF.
    pub fn remove_tracer<T>(&mut self, target: Handle<T>)
    where
        T: Trace,
    {
        let addr = target.as_addr();
        debug_assert!(self.trace_targets.contains_key(&addr));
        self.trace_targets.remove(&addr);
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
            if let Some(tracer) = self.trace_targets.get(&addr) {
                (tracer.trace_fn)(addr, &mut state.visit_list);
            }
        }
    }

    /// Performs the sweep phase.
    fn sweep(&mut self, state: &mut GcState) {
        for (addr, memory) in self
            .memory_blocks
            .extract_if(|addr, _| !state.visited.contains(addr))
        {
            self.trace_targets.remove(&addr);
            if let Some(tidy_fn) = memory.tidy_fn {
                tidy_fn(addr);
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
            num_objects: self.memory_blocks.len(),
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        for (addr, memory) in self.memory_blocks.iter() {
            // TODO(refactor): code clone
            if let Some(tidy_fn) = memory.tidy_fn {
                tidy_fn(*addr);
            }
            // SAFETY: the memory block was allocated by using std::alloc::alloc().
            unsafe {
                std::alloc::dealloc(*addr as *mut u8, memory.layout);
            }
        }
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

struct MemoryBlock {
    layout: Layout,
    tidy_fn: Option<TidyFn>,
}

impl MemoryBlock {
    fn new<T>() -> Self {
        Self::with_layout::<T>(Layout::new::<T>())
    }

    fn with_layout<T>(layout: Layout) -> Self {
        Self {
            layout,
            tidy_fn: if std::mem::needs_drop::<T>() {
                Some(|addr| {
                    // SAFETY: `addr` is always valid.
                    unsafe {
                        std::ptr::drop_in_place(addr as *mut T);
                    }
                })
            } else {
                None
            },
        }
    }
}

struct Tracer {
    trace_fn: TraceFn,
}

impl Tracer {
    fn new<T: Trace>() -> Self {
        Self {
            trace_fn: |addr, visits| {
                // SAFETY: `addr` is always valid.
                let reciever = unsafe { &*(addr as *const T) };
                reciever.trace(visits);
            },
        }
    }
}

type TidyFn = fn(usize);
type TraceFn = fn(usize, &mut VisitList);

pub trait Trace {
    fn trace(&self, visits: &mut VisitList);
}

pub trait Atom: Copy + Sized {}
impl Atom for u16 {}
