pub struct NewType<T>(Vec<T>);

impl<T> NewType<T> {

    pub fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }

    pub fn iter(&self) -> NewTyperIter<T> {
        NewTyperIter {
            inner: self,
            pos: 0,
        }
    }

    pub fn iter_mut(&mut self) -> NewTypeIterMut<T>{
        NewTypeIterMut { inner: self, default: NewType::from(vec![]), pos: 0 }
    }
}

pub struct NewTyperIter<'a, T> {
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

pub struct NewTypeIterMut<'b, T> {
    inner: &'b mut NewType<T>,
    default: NewType<T>,
    pos: usize,
}

impl<'b, T> Iterator for NewTypeIterMut<'b, T> {
    type Item = &'b mut T;

    fn next(&mut self) -> Option<Self::Item> {
        
        let inner_field = std::mem::replace(&mut self.inner, Vec::);

        if self.pos <= self.inner.0.len() {
            self.pos += 1;

            return inner_field.0.get_mut(self.pos - 1);
        }

        None

    }
}