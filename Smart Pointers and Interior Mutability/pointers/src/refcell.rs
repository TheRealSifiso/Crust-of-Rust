/*
https://doc.rust-lang.org/std/cell/struct.RefCell.html:

A mutable memory location with "safe" dynamically check borrow rules!

The checks are done at runtime!

*/

use std::cell::UnsafeCell;
use crate::cell::Cell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared, //T
    Shared(usize), //&T
    Exclusive, //&mut T
}

//Use cell to mutate behind a shared reference
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    fn borrow(&self) -> Option<&T> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some (unsafe {&*self.value.get()})
            },
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(unsafe {&*self.value.get()})
            },
            _ => None ,
        }
    }

    fn borrow_mut(&self) -> Option<&mut T> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(unsafe {&mut *self.value.get()})
        } else {
            None
        }
    }
 }