use fnv::FnvHasher;
use std::cmp::Ordering;
use std::hash::{Hash,Hasher};
use std::thread;

/// Prio guarantees a *total* order, even though the values provided by the user might only be
/// partially ordered.  It does this by also comparing on ThreadId.
///
/// Assumptions: only one Prio per thread; no hash collisions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prio {
    prio: usize,
    thread_hash: u64,
}

impl Prio {
    pub fn new(prio: usize) -> Prio {
        let mut s = FnvHasher::default();
        thread::current().id().hash(&mut s);
        Prio {
            prio: prio,
            thread_hash: s.finish(),
        }
    }
}


/// A value `V` with a priority `P`
pub struct PV<P,V> {
    pub p: P,
    pub v: V,
}

impl<P:PartialEq,V> PartialEq for PV<P,V> {
    fn eq(&self, other: &PV<P,V>) -> bool { other.p == self.p }
}

impl<P:Eq,V> Eq for PV<P,V> {}

impl<P:PartialOrd,V> PartialOrd for PV<P,V> {
    fn partial_cmp(&self, other: &PV<P,V>) -> Option<Ordering> { other.p.partial_cmp(&self.p) }
}

impl<P:Ord,V> Ord for PV<P,V> {
    fn cmp(&self, other: &PV<P,V>) -> Ordering { other.p.cmp(&self.p) }
}