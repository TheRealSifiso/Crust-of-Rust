/*

An Rc (short for Reference Counted) is a single-threaded reference-counting
pointer.

Rc<T> provides shared ownership of a value of type 'T', allocated on the
heap.

Invoking 'clone' on 'Rc' produces a new pointer to the same allocation in
the heap. When the last 'Rc' pointer to a given allocation is destroyed,
the value stored in that allocation (the "inner value") is also dropped.

Since shared references (including 'Rc') disallow mutation by default,
you need to put a 'Cell' or 'RefCell' inside the 'Rc' to obtain mutability.

Remember: Cell<T> gives you a values; where as, RefCell<T> gives you
references.

Since 'Rc' uses non-atomic reference counting, overhead is very low. However,
an 'Rc' cannot be sent between threads, thus Rc<T> does not implement 'Send'.
*/

/*
If Rc<T> had a field for 'reference count', then cloning Rc<T> would also
clone 'refcount'. In addition to that, if Rc<T> had a value field, then
cloning Rc<T> would also clone the value as opposed to raising 'refcount'.

Both 'value' and 'refcount' are, therefore, stored in RcInner<T>.
*/

use super::cell::Cell;
struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

/*
Since both 'value' and 'refcount' are stored in RcInner<T>, Rc<T> stores
a raw pointer to RcInner<T>.

We use *const
*/
pub struct Rc<T> {
    inner: *const RcInner<T>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        //Box<T> gives you heap memory allocation and Indirection.
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });
        
        /*
        Box::into_raw():
            pub fn into_raw(b: Self) -> *mut T

        into_raw() consumes the box and returns a raw pointer to the inner
        value (type T)

        Rc {inner: &*inner} wouldn't work because at the end of the
        function scope, the box is dropped and memory is deallocated. This
        would result in a dangling reference.
         */
        Rc {
            inner: Box::into_raw(inner),
        }
    }
}

/*
Cloning an Rc<T> does not 'copy' the inner value; it only increases the
'reference count'
*/
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &*self.inner };
        let c = inner.refcount.get();

        /*
        We had to encapsulate 'refcount' in Cell<T> to obtain interior
        mutability, since we cannot mutate behind a shared reference.
        */
        inner.refcount.set(c + 1);
        Rc { inner: self.inner }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        /*
        SAFETY: self.inner is a Box that that is only deallocated when the
        last Rc goes away.
        We use an Rc, there the Box has not been deallocated, so deref is 
        fine.
        */
        &unsafe { &*self.inner }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { &*self.inner };
        let c = inner.refcount.get();

        if c == 1 {
            /*
            SAFETY: we are the only Rc left, and we are being dropped.
            Therefore, after us, there will be no Rc's, and no references
            to T.
            */
            drop(inner);

            //let _ = Box::from_raw(self.inner);
        } else {
            //There are other Rc's, so don't drop the Box!
            inner.refcount.set(c-1);
            /*
            When an Rc<T> is dropped, 'refcount' decrements.
            */
        }
    }
}
