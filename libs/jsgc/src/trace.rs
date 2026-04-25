use std::collections::VecDeque;

pub trait Trace {
    fn trace(&self, visits: &mut VisitList);
}

impl<T> Trace for Option<T>
where
    T: Trace,
{
    #[inline]
    fn trace(&self, visits: &mut VisitList) {
        if let Some(v) = self {
            v.trace(visits)
        }
    }
}

macro_rules! impl_trace_empty {
    ($ty:ty) => {
        impl Trace for $ty {
            #[inline]
            fn trace(&self, _visits: &mut VisitList) {}
        }
    };
    ($ty:ty, $($rest:ty),+) => {
        impl_trace_empty! { $ty }
        impl_trace_empty! { $($rest),+ }
    };
    ($($valiadic:ty,)+) => {
        impl_trace_empty! { $($valiadic),+ }
    };
}

impl_trace_empty! {
    (),
    bool,
    char,
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    str,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
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
    pub(crate) fn pop(&mut self) -> Option<usize> {
        self.0.pop_front()
    }
}
