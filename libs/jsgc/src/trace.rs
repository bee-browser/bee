use std::collections::VecDeque;

pub trait Trace {
    fn trace(&self, visits: &mut VisitList);
}

impl<T: Atom> Trace for T {
    #[inline]
    fn trace(&self, _visits: &mut VisitList) {}
}

impl<T: Trace> Trace for Option<T> {
    #[inline]
    fn trace(&self, visits: &mut VisitList) {
        if let Some(v) = self {
            v.trace(visits)
        }
    }
}

pub trait Atom: Copy + Sized {}

impl Atom for () {}
impl Atom for bool {}
impl Atom for u16 {}
impl Atom for u32 {}

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
