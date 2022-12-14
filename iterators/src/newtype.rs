pub struct NewType<T>(Vec<T>);

impl<T> NewType<T> {
    fn iter(&mut self) -> NewTypeIter<T> {
        NewTypeIter {
            inner: self,
            pos: 0,
        }
    }
}

struct NewTypeIter<'a, T> {
    inner: &'a mut NewType<T>,
    pos: usize,
}

impl<'a, T> Iterator for NewTypeIter<'a, T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos <= self.inner.0.len() {
            self.pos += 1;
            return self.inner.0.get(self.pos-1);
        }

        None
    }
}

#[test]

fn it_works(){
    let mut instance = NewType(vec![1, 2, 3, 4]);

    //instance.iter()
    assert_eq!(instance.iter().collect::<Vec<&i32>>(), vec![&1, &2, &3, &4]);
}