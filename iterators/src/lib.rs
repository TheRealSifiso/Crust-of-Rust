fn flatten<O>(iter: O) -> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    Flatten {
        outer_iter: iter,
        forward_iter: None,
        backward_iter: None,
    }
}

struct Flatten<O>
where
    O: Iterator,
    <O as Iterator>::Item: IntoIterator,
{
    outer_iter: O,
    forward_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    backward_iter: Option<<O::Item as IntoIterator>::IntoIter>,
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
            //Check if forward_inner_iter exists
            if let Some(ref mut forward_inner_iter) = self.forward_iter {
                //Check if there are any items to be yielded by the forward_inner_iter
                if let Some(item) = forward_inner_iter.next() {
                    return Some(item);
                }

                self.forward_iter = None;
            }

            if let Some(inner_collection) = self.outer_iter.next() {
                self.forward_iter = Some(inner_collection.into_iter());
            } else {
                return self.backward_iter.as_mut()?.next();
            }
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
            //Check if forward_inner_iter exists
            if let Some(ref mut backward_inner_iter) = self.backward_iter {
                //Check if there are any items to be yielded by the forward_inner_iter
                if let Some(item) = backward_inner_iter.next_back() {
                    return Some(item);
                }

                self.backward_iter = None;
            }

            if let Some(inner_collection) = self.outer_iter.next_back() {
                self.backward_iter = Some(inner_collection.into_iter());
            } else {
                return self.forward_iter.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

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

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]].into_iter())
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]].into_iter());

        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
    }
}
