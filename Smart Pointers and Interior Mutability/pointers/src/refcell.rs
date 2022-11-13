/*
https://doc.rust-lang.org/std/cell/struct.RefCell.html:

A mutable memory location with "safe" dynamically check borrow rules!

The checks are done at runtime!

*/

use std::cell::UnsafeCell;

enum RefState {
    Unshared, //T
    Shared(usize), //&T
    Exclusive, //&mut T
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: RefState,
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: RefState::Unshared,
        }
    }

    fn borrow(&self) -> Option<&T> {
        match self.state {
            RefState::Unshared => Some (unsafe {&*self.value.get()}),
            RefState::Shared(_) => Some(unsafe {&*self.value.get()}),
            _ => None ,
        }
    }

    fn borrow_mut(&self) -> Option<&mut T> {
        if let RefState::Unshared = self.state {
            Some(unsafe {&mut *self.value.get()})
        } else {
            None
        }
    }
 }