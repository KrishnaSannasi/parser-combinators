use std::collections::*;
use std::hash::{BuildHasher, Hash};

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

pub trait Collection<Item> {
    fn put(&mut self, i: Item);

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;
}

impl<Item> Collection<Item> for () {
    #[inline]
    fn put(&mut self, _: Item) {}

    #[inline]
    fn len(&self) -> usize {
        0
    }
}

impl Collection<char> for String {
    #[inline]
    fn put(&mut self, c: char) {
        self.push(c)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<Item> Collection<Item> for Vec<Item> {
    #[inline]
    fn put(&mut self, item: Item) {
        self.push(item)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<Key: Hash + Eq, Value, S: BuildHasher> Collection<(Key, Value)> for HashMap<Key, Value, S> {
    #[inline]
    fn put(&mut self, (key, value): (Key, Value)) {
        self.insert(key, value);
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<Item: Hash + Eq, S: BuildHasher> Collection<Item> for HashSet<Item, S> {
    #[inline]
    fn put(&mut self, item: Item) {
        self.insert(item);
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<Key: Ord, Value> Collection<(Key, Value)> for BTreeMap<Key, Value> {
    #[inline]
    fn put(&mut self, (key, value): (Key, Value)) {
        self.insert(key, value);
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<Item: Ord> Collection<Item> for BTreeSet<Item> {
    #[inline]
    fn put(&mut self, item: Item) {
        self.insert(item);
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<Item, C: ?Sized + Collection<Item>> Collection<Item> for Box<C> {
    #[inline]
    fn put(&mut self, item: Item) {
        C::put(self, item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        C::is_empty(self)
    }

    #[inline]
    fn len(&self) -> usize {
        C::len(self)
    }
}

impl<Item, C: ?Sized + Collection<Item>> Collection<Item> for &mut C {
    #[inline]
    fn put(&mut self, item: Item) {
        C::put(self, item)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        C::is_empty(self)
    }

    #[inline]
    fn len(&self) -> usize {
        C::len(self)
    }
}

impl<Item, C: ?Sized> Collection<Item> for Rc<C>
where
    for<'a> &'a C: Collection<Item>,
{
    #[inline]
    fn put(&mut self, item: Item) {
        <&C>::put(&mut &**self, item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        <&C>::is_empty(&&**self)
    }

    #[inline]
    fn len(&self) -> usize {
        <&C>::len(&&**self)
    }
}

impl<Item, C> Collection<Item> for Arc<C>
where
    for<'a> &'a C: Collection<Item>,
{
    #[inline]
    fn put(&mut self, item: Item) {
        <&C>::put(&mut &**self, item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        <&C>::is_empty(&&**self)
    }

    #[inline]
    fn len(&self) -> usize {
        <&C>::len(&&**self)
    }
}

impl<Item, C: Default + Collection<Item>> Collection<Item> for Cell<C> {
    #[inline]
    fn put(&mut self, item: Item) {
        C::put(self.get_mut(), item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let c = self.take();
        let out = C::is_empty(&c);
        self.set(c);
        out
    }

    #[inline]
    fn len(&self) -> usize {
        let c = self.take();
        let out = C::len(&c);
        self.set(c);
        out
    }
}

impl<Item, C: Default + Collection<Item>> Collection<Item> for &Cell<C> {
    #[inline]
    fn put(&mut self, item: Item) {
        let mut c = self.take();
        C::put(&mut c, item);
        self.set(c);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let c = self.take();
        let out = C::is_empty(&c);
        self.set(c);
        out
    }

    #[inline]
    fn len(&self) -> usize {
        let c = self.take();
        let out = C::len(&c);
        self.set(c);
        out
    }
}

impl<Item, C: ?Sized + Collection<Item>> Collection<Item> for RefCell<C> {
    #[inline]
    fn put(&mut self, item: Item) {
        C::put(self.get_mut(), item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        C::is_empty(&self.borrow())
    }

    #[inline]
    fn len(&self) -> usize {
        C::len(&self.borrow())
    }
}

impl<Item, C: ?Sized + Collection<Item>> Collection<Item> for &RefCell<C> {
    #[inline]
    fn put(&mut self, item: Item) {
        C::put(&mut self.borrow_mut(), item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        C::is_empty(&self.borrow())
    }

    #[inline]
    fn len(&self) -> usize {
        C::len(&self.borrow())
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for RwLock<C> {
    #[inline]
    fn put(&mut self, item: Item) {
        C::put(self.get_mut().unwrap(), item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        C::is_empty(&self.read().unwrap())
    }

    #[inline]
    fn len(&self) -> usize {
        C::len(&self.read().unwrap())
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for &RwLock<C> {
    #[inline]
    fn put(&mut self, item: Item) {
        C::put(&mut self.write().unwrap(), item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        C::is_empty(&self.read().unwrap())
    }

    #[inline]
    fn len(&self) -> usize {
        C::len(&self.read().unwrap())
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for Mutex<C> {
    #[inline]
    fn put(&mut self, item: Item) {
        C::put(self.get_mut().unwrap(), item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        C::is_empty(&self.lock().unwrap())
    }

    #[inline]
    fn len(&self) -> usize {
        C::len(&self.lock().unwrap())
    }
}

impl<Item, C: Collection<Item>> Collection<Item> for &Mutex<C> {
    #[inline]
    fn put(&mut self, item: Item) {
        C::put(&mut self.lock().unwrap(), item);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        C::is_empty(&self.lock().unwrap())
    }

    #[inline]
    fn len(&self) -> usize {
        C::len(&self.lock().unwrap())
    }
}
