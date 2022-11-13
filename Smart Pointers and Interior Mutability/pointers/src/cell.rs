use std::cell::UnsafeCell;

pub struct Cell<T> {
    /*
    https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html:

    UnsafeCell is the core primitive for interior mutability in Rust.

    Mutating data through an alias (an immutable reference) or by
    transmuting an &T into an &mut is considered undefined behavior.

    A shared reference &UnsafeCell<T> may point to data that is being
    mutated. This defines "interior mutability"!

    UnsafeCell<T> is used by all other types, such as Cell<T> and
    RefCell<T>, that allow internal mutability.

    There is no legal way to obtain aliasing &mut, not even with
    UnsafeCell<T>. Thus, the only way to transmute a shared reference
    into an exclusive one in Rust is by making use of UnsafeCell<T>.

    */
    value: UnsafeCell<T>,
}

/*
impl<T> !Sync for Cell<T> {}
is implied by UnsafeCell<T>
*/

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        /*
        The following unsafe operation assumes that we know no-one else
        is concurrently mutating self.value. This assumption is true
        because the !Sync implementation is implied by UnsafeCell<T>.

        This implies Cell<T> cannot be used beyond thread boundaries,
        thus preventing Data Races.
        */
        unsafe { *self.value.get() = value };
    }

    /*
    We cannot move out of a reference. The return type must implement
    the Copy trait.

    The idiomatic Rust way of adding bounds is to add them only where
    they are necessary.

    Note: every Copy type is also required to be Clone. However, they
    are not required to do the same thing!
    */
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}