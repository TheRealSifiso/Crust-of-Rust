mod flatten;
mod newtypeiterators;

#[cfg(test)]
mod flatten_tests {

    use std::vec;

    use crate::flatten::flatten;

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

#[cfg(test)]
mod newtypeiterators_tests {

    use crate::newtypeiterators::NewType;

    #[test]
    fn newtypeiter(){
        let collection = NewType::from(vec![1, 2, 3, 4]);

        assert_eq!(collection.iter().collect::<Vec<&i32>>(), vec![&1, &2, &3, &4]);
    }

    #[test]
    fn newtypeitermut(){
        let mut collection = NewType::from(vec![1, 2, 3, 4]);

        assert_eq!(collection.iter_mut().collect::<Vec<&mut i32>>(), vec![&mut 1, &mut 2, &mut 3, &mut 4]);
    }

}