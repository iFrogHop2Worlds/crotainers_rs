use crotainers_rs::etc::BinCroHeap;
use crotainers_rs::maps::{CroBTree, CroMap};
use crotainers_rs::sequences::{CroLList, CroQue, CroVec};
use crotainers_rs::sets::{CroBTreeSet, CroHashSet};

fn main() {
    let mut v = CroVec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("CroVec size={}, first={:?}, last={:?}", v.size(), v.first(), v.last());

    let mut q = CroQue::new();
    q.push_back("middle");
    q.push_front("front");
    q.push_back("back");
    println!("CroQue pop_front={:?}, pop_back={:?}", q.pop_front(), q.pop_back());

    let mut ll = CroLList::new();
    ll.push_back(1);
    ll.push_back(2);
    ll.push_front(0);
    println!("CroLList pop_front={:?}, pop_back={:?}", ll.pop_front(), ll.pop_back());

    let mut map = CroMap::new();
    map.insert("alice", 42);
    map.insert("bob", 7);
    println!("CroMap alice={:?}, size={}", map.get(&"alice"), map.size());

    let mut bt = CroBTree::new();
    bt.insert(2, "two");
    bt.insert(1, "one");
    bt.insert(3, "three");
    println!("CroBTree key=2 => {:?}", bt.get(&2));

    let mut hs = CroHashSet::new();
    hs.insert("red");
    hs.insert("green");
    println!("CroHashSet contains red? {}", hs.contains(&"red"));

    let mut bts = CroBTreeSet::new();
    bts.insert(5);
    bts.insert(1);
    bts.insert(9);
    println!("CroBTreeSet first={:?}, last={:?}", bts.first(), bts.last());

    let mut heap = BinCroHeap::new();
    heap.push(4);
    heap.push(10);
    heap.push(7);
    println!("BinCroHeap peek={:?}", heap.peek());
    while let Some(top) = heap.pop() {
        println!("heap pop -> {}", top);
    }
}
