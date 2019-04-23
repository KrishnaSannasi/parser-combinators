
use std::collections::*;
use std::hash::{Hash, BuildHasher};

use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex, RwLock};
use std::rc::Rc;

pub trait Collection<Item> {
    fn put(&mut self, i: Item);

    fn is_empty(&self) -> bool { self.len() == 0 }

    fn len(&self) -> usize;
}

impl<Item> Collection<Item> for () {
    fn put(&mut self, _: Item) {}

    fn len(&self) -> usize { 0 }
}

impl Collection<char> for String {
    fn put(&mut self, c: char) { self.push(c) }

    fn len(&self) -> usize { self.len() }
}

impl<Item> Collection<Item> for Vec<Item> {
    fn put(&mut self, item: Item) { self.push(item) }

    fn len(&self) -> usize { self.len() }
}

impl<Key: Hash + Eq, Value, S: BuildHasher> Collection<(Key, Value)> for HashMap<Key, Value, S> {
    fn put(&mut self, (key, value): (Key, Value)) { self.insert(key, value); }

    fn len(&self) -> usize { self.len() }
}

impl<Item: Hash + Eq, S: BuildHasher> Collection<Item> for HashSet<Item, S> {
    fn put(&mut self, item: Item) { self.insert(item); }

    fn len(&self) -> usize { self.len() }
}

impl<Key: Ord, Value> Collection<(Key, Value)> for BTreeMap<Key, Value> {
    fn put(&mut self, (key, value): (Key, Value)) { self.insert(key, value); }

    fn len(&self) -> usize { self.len() }
}

impl<Item: Ord> Collection<Item> for BTreeSet<Item> {
    fn put(&mut self, item: Item) { self.insert(item); }

    fn len(&self) -> usize { self.len() }
}

impl<Item, C: Collection<Item>> Collection<Item> for Box<C> {
    fn put(&mut self, item: Item) {
        C::put(self, item);
    }
    
    fn is_empty(&self) -> bool {
        C::is_empty(self)
    }
    
    fn len(&self) -> usize {
        C::len(self)
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for &mut C {
    fn put(&mut self, item: Item) { C::put(self, item) }
    
    fn is_empty(&self) -> bool { C::is_empty(self) }
    
    fn len(&self) -> usize { C::len(self) }
}

impl<Item, C> Collection<Item> for Rc<C> 
where for<'a> &'a C: Collection<Item> {
    fn put(&mut self, item: Item) {
        <&C>::put(&mut &**self, item);
    }
    
    fn is_empty(&self) -> bool {
        <&C>::is_empty(&&**self)
    }
    
    fn len(&self) -> usize {
        <&C>::len(&&**self)
    }
}

impl<Item, C> Collection<Item> for Arc<C> 
where for<'a> &'a C: Collection<Item> {
    fn put(&mut self, item: Item) {
        <&C>::put(&mut &**self, item);
    }
    
    fn is_empty(&self) -> bool {
        <&C>::is_empty(&&**self)
    }
    
    fn len(&self) -> usize {
        <&C>::len(&&**self)
    }
}

impl<Item, C: Default + Collection<Item>> Collection<Item> for Cell<C> {
    fn put(&mut self, item: Item) {
        C::put(self.get_mut(), item);
    }
    
    fn is_empty(&self) -> bool {
        let c = self.take();
        let out = C::is_empty(&c);
        self.set(c);
        out
    }
    
    fn len(&self) -> usize {
        let c = self.take();
        let out = C::len(&c);
        self.set(c);
        out
    }
}

impl<Item, C: Default + Collection<Item>> Collection<Item> for &Cell<C> {
    fn put(&mut self, item: Item) {
        let mut c = self.take();
        C::put(&mut c, item);
        self.set(c);
    }
    
    fn is_empty(&self) -> bool {
        let c = self.take();
        let out = C::is_empty(&c);
        self.set(c);
        out
    }
    
    fn len(&self) -> usize {
        let c = self.take();
        let out = C::len(&c);
        self.set(c);
        out
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for RefCell<C> {
    fn put(&mut self, item: Item) {
        C::put(self.get_mut(), item);
    }
    
    fn is_empty(&self) -> bool {
        C::is_empty(&self.borrow())
    }
    
    fn len(&self) -> usize {
        C::len(&self.borrow())
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for &RefCell<C> {
    fn put(&mut self, item: Item) {
        C::put(&mut self.borrow_mut(), item);
    }
    
    fn is_empty(&self) -> bool {
        C::is_empty(&self.borrow())
    }
    
    fn len(&self) -> usize {
        C::len(&self.borrow())
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for RwLock<C> {
    fn put(&mut self, item: Item) {
        C::put(self.get_mut().unwrap(), item);
    }
    
    fn is_empty(&self) -> bool {
        C::is_empty(&self.read().unwrap())
    }
    
    fn len(&self) -> usize {
        C::len(&self.read().unwrap())
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for &RwLock<C> {
    fn put(&mut self, item: Item) {
        C::put(&mut self.write().unwrap(), item);
    }
    
    fn is_empty(&self) -> bool {
        C::is_empty(&self.read().unwrap())
    }
    
    fn len(&self) -> usize {
        C::len(&self.read().unwrap())
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for Mutex<C> {
    fn put(&mut self, item: Item) {
        C::put(self.get_mut().unwrap(), item);
    }
    
    fn is_empty(&self) -> bool {
        C::is_empty(&self.lock().unwrap())
    }
    
    fn len(&self) -> usize {
        C::len(&self.lock().unwrap())
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for &Mutex<C> {
    fn put(&mut self, item: Item) {
        C::put(&mut self.lock().unwrap(), item);
    }
    
    fn is_empty(&self) -> bool {
        C::is_empty(&self.lock().unwrap())
    }
    
    fn len(&self) -> usize {
        C::len(&self.lock().unwrap())
    }
}
