fn flatten<O>(iter: O) -> Flatten<O> {
    Flatten { outer: iter }
}

struct Flatten<O> {
    outer: O,
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: Iterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/*

let vec = vec![vec[]!, vec[]!]

*/
