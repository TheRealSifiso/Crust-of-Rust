struct NewType<T>(Vec<T>);

impl<T> NewType<T> {
    fn iter(&self) -> NewTyperIter<T> {
        NewTyperIter {
            inner: self,
            pos: 0,
        }
    }
}

struct NewTyperIter<'a, T> {
    inner: &'a NewType<T>,
    pos: usize,
}

impl<'a, T> Iterator for NewTyperIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos <= self.inner.0.len() {

            self.pos += 1;
            
            return self.inner.0.get(self.pos - 1);
        }

        None
    }
}
