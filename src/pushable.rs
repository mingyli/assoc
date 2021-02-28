use std::collections::{LinkedList, VecDeque};
pub trait Pushable<T> {
    fn push(&mut self, t: T);
    fn last_mut(&mut self) -> Option<&mut T>;
}

impl<T> Pushable<T> for Vec<T> {
    fn push(&mut self, t: T) {
        Vec::push(self, t)
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        <[T]>::last_mut(self)
    }
}

impl<T> Pushable<T> for VecDeque<T> {
    fn push(&mut self, t: T) {
        VecDeque::push_back(self, t)
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        VecDeque::back_mut(self)
    }
}

impl<T> Pushable<T> for LinkedList<T> {
    fn push(&mut self, t: T) {
        LinkedList::push_back(self, t)
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        LinkedList::back_mut(self)
    }
}
