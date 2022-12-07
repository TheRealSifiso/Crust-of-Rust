pub fn flatten<O>(iter: O) -> Flatten<O> {
    Flatten::new(iter)
}

pub struct Flatten<O>{
    outer: O,
}

impl<O> Flatten<O> {
    fn new(iter: O) -> Self {
        Self {
            outer: iter,
        }
    }
}

/*

let iter = vec![vec![], vec![], vec![]];

flatten(iter) -> Flatten<O>

for item in Flatten<O> {} -> flatten the iterator

*/