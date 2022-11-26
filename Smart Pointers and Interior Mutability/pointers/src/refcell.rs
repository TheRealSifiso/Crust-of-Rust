/*
https://doc.rust-lang.org/std/cell/struct.RefCell.html:

A mutable memory location with "safe" dynamically checked borrow rules!

The checks are done at runtime to ensure certain memory-safe scenarios
are allowed, but if the rules are broken, the program will panic and exit.

Cell<T> will allow you to replace its value using the set() method, 
but unlike RefCell<T>, it will not give you an exclusive reference (&mut)
to that value.

*/

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize), //We need to keep track of the number of shared references
    Exclusive,
}

use crate::cell::Cell;
use std::cell::UnsafeCell;

/*
RefCell<T> contains a value of type 'T' wrapped in UnsafeCell<T> - the core
primitive for interior mutability in Rust, and a state of type 'RefState' (enum)
wrapped in Cell<T> - a mutable memory location - which provides methods
to retrieve change the current interior value.

We wrap 'RefState' in Cell<T> because each time call borrow() or borrow_mut()
we take a shared reference to RefCell<T> (&self), match the state field,
and then change the state based on the borrowing rules enforced at
runtime. Now, suppose borrow() and borrow_mut() take exclusive references
to RefCell<T> (&mut self). It's clear that this would violate the borrowing
rules which we have enforced an runtime. This the perfect case for the use
of Cell<T> (Cell<RefState>) - to obtain interior mutability.

RefCell<T> keeps track of the number of active Ref<T> and RefMut<T> smart
pointers.

Borrowing rules: one exclusive reference (&mut) at a time or any number of
shared references (&).
*/
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            //Initially, the value is 'unshared'.
            state: Cell::new(RefState::Unshared),
        }
    }

    /*
    
    borrow() returns Ref<T> (a smart pointer) which gives us an immutable
    reference to the value inside of RefCell<T>.

    */
    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                //SAFETY: no exclusive references given out
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                //SAFETY: no exclusive references given out
                Some(Ref { refcell: self })
            }
            //RefState::Exclusive => None,
            _ => None,
        }
    }

    /*
    
    borrow_mut() returns RefMut<T> (a smart pointer) which gives us a
    mutable reference to the value inside of RefCell<T>

    */

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            //SAFETY: no other references exist
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

/*

Ref<T> contains a reference to RefCell<T>. If RefCell<T> gets dropped, then
all Ref<T> and RefMut<T> smart pointers must get dropped too otherwise Ref<T>
and RefMut<T> will contain dangling pointers. This explains the contract of both
smart pointers: Ref<'refcell, T> and RefMut<'refcell, T>. Both have a lifetime
specifier that ties the lifetime of the smart pointers to the liftime of the reference
to RefCell<T>.

*/
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

use std::ops::{Deref, DerefMut};

/*

When you call borrow(), you get back Ref<T> provided that there is no
active RefMut<T> smart pointer (thus, no exclusive reference to the value
contained in RefCell<T>). This simply means that, when dropped, we do not
consider an 'Exclusive' or 'Unshared' state, hence the 'unreachable!()'
macro which indicates unreachable code.

"This is useful any time that the compiler can’t determine that some code 
is unreachable... If the determination that the code is unreachable proves 
incorrect, the program immediately terminates with a panic!."

*/
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
        }
    }
}

/*

For immutable dereferencing operations, the Deref trait - implemented for
Ref<'_, T> - enables us to treat Ref<'_, T> like a regular reference.

Dereferencing Ref<'_, T> returns a shared reference to the value in
RefCell<T>.

*/
impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}


impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}

/*
For immutable dereferencing operations, the Deref trait - implemented for
RefMut<'_, T> - enables us to treat RefMut<'_, T> like a regular reference.

Dereferencing RefMut<'_, T> returns a shared reference to the value in
RefCell<T>.

*/

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

/*
Note: You need to implement 'Deref' before you can implement 'DerefMut'.

For mutable dereferencing operations, the DerefMut trait - implemented for
RefMut<'_, T> - enables us to treat RefMut<'_, T> like a regular reference,
in a mutable context.

Dereferencing RefMut<'_, T>, in a mutable context, returns an exclusive reference
to the value in RefCell<T>.

*/

impl<T> DerefMut for RefMut<'_, T> {
    /*
    You do not need "type Target = T" because it's already specified in
    the implementation of Deref.
    */

    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}