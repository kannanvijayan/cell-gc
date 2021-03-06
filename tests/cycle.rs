//! Test that the GC is not confused by cycles in the reachable object graph.

extern crate cell_gc;
#[macro_use]
extern crate cell_gc_derive;
mod aux;
use aux::pairs::*;

#[test]
fn cycle() {
    cell_gc::with_heap(|hs| {
        // Set up obj1 and obj2 to point to each other.
        let obj1 = alloc_null_pair(hs);
        let obj2 = alloc_pair(hs, Value::Pair(obj1.clone()), Value::Pair(obj1.clone()));
        obj1.set_head(Value::Pair(obj2.clone()));
        obj1.set_tail(Value::Pair(obj2.clone()));

        hs.force_gc();

        // After GC, the two objects are unchanged.
        assert_eq!(obj1.head(), Value::Pair(obj2.clone()));
        assert_eq!(obj1.tail(), Value::Pair(obj2.clone()));
        assert_eq!(obj2.head(), Value::Pair(obj1.clone()));
        assert_eq!(obj2.tail(), Value::Pair(obj1.clone()));
    });
}
