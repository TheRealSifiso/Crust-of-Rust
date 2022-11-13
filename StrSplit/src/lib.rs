//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack> Iterator for StrSplit<'haystack, '_> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        
        if let Some(ref mut remainder) = self.remainder
        /*Option<&'a str>*/
        {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delim = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delim)
            } else {
                //impl<T> Option<T> { fn take(&mut self) -> Option<T>}
                let rest = self.remainder;
                self.remainder = None;
                rest

                //Better alternative:
                //self.remainder.take() - return Some(T) and leave None in place.
            }
        } else {
            None
        }
    }
}

pub fn until_char<'haystack> (s: &'haystack str, c: char) -> &'haystack str {
    StrSplit::new(s, &c.to_string())
        .next()
        .expect("StrSplit always gives at least one result")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tails() {
        let haystack = "a b c d e ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);
    }

    #[test]
    fn until_char_test(){
        assert_eq!(until_char("Hello, World!", 'l'), "He");
    }
}
