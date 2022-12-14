pub struct NewType<T>(Vec<T>);

impl<T> NewType<T> {
    fn iter(&self) -> NewTypeIter<T> {
        NewTypeIter {
            inner: self,
            pos: 0,
        }
    }
}

struct NewTypeIter<'a, T> {
    inner: &'a NewType<T>,
    pos: usize,
}
