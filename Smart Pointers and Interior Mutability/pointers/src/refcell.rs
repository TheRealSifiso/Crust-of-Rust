/*
https://doc.rust-lang.org/std/cell/struct.RefCell.html:

A mutable memory location with "safe" dynamically checked borrow rules!

The checks are done at runtime!

Cell<T> will allow you to replace its value, but unlike RefCell<T>,
it will not give you an exclusive reference (&mut) to that value.

*/

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

use crate::cell::Cell;
use std::cell::UnsafeCell;
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self{
        Self{
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref {refcell: self})
            } RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref {refcell: self})
            },
            _ => None,
        }
    }

    fn borrow_mut(&self) -> Option <RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut {refcell: self})
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

use std::ops::{Deref, DerefMut};

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n-1)),
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *self.refcell.value.get()}
    }
}