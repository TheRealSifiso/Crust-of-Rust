fn flatten<O>(iter: O) -> Flatten<O> {
    Flatten { outer_iter: iter }
}

struct Flatten<O> {
    outer_iter: O,
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    // 'O' -> outer iterator
    // 'O::Item' -> collection yielded by the call to next() on 'O' (the 'outer' iterator)
    // 

    fn next(&mut self) -> Option<Self::Item> {
          self.outer_iter.next()?.into_iter().next()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty(){
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide(){
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(), 0);
    }

    #[test]
    fn one(){
        assert_eq!(flatten(std::iter::once(vec!["a", "b", "c"])).count(), 1);
    }

    #[test]
    fn two(){
        assert_ne!(flatten(std::iter::once(vec!["a", "b", "c"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2);
    }

}