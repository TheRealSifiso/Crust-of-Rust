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
          self.outer_iter.next().and_then(|inner_collection| inner_collection.into_iter().next())
    }
}