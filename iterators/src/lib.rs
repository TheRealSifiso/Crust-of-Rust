fn flatten<O>(iter: O) -> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    Flatten {
        outer_iter: iter,
        inner_iter: None,
    }
}

struct Flatten<O>
where
    O: Iterator,
    <O as Iterator>::Item: IntoIterator,
{
    outer_iter: O,
    inner_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    // 'O' -> outer iterator
    // 'O::Item' -> collection yielded by the call to next() on 'O' (the 'outer' iterator)
    // [[], [], []]

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            //Check whether inner iter exists
            if let Some(ref mut inner_iter) = self.inner_iter {
                //Check whether inner iter has been exhausted, or has any items to yield
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }

                self.inner_iter = None;
            }

            self.inner_iter = Some(self.outer_iter.next()?.into_iter());
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator, // may ommit 'Iterator' because it is implied by 'DoubleEndedIterator'
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            //Check whether inner iter exists
            if let Some(ref mut inner_iter) = self.inner_iter {
                //Check whether inner iter has been exhausted, or has any items to yield
                if let Some(item) = inner_iter.next_back() {
                    return Some(item);
                }

                self.inner_iter = None;
            }

            self.inner_iter = Some(self.outer_iter.next_back()?.into_iter());
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        );
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b", "c"])).count(), 3);
    }

    #[test]
    fn two() {
        assert_ne!(flatten(std::iter::once(vec!["a", "b", "c"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2);
    }
}
