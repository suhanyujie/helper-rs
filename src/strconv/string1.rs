//! 字符串处理
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use std::ops::RemAssign;

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        // let remainder /* &mut &str */= &mut self.remainder?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }

        // if let Some(next_delim) = self.remainder.find(self.delimiter) {
        //     let until_delimiter = &self.remainder[..next_delim];
        //     self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
        //     Some(until_delimiter)
        // } else if self.remainder.is_empty() {
        //     // todo fix bug
        //     None
        // } else {
        //     let rest = self.remainder;
        //     self.remainder = "";
        //     Some(rest)
        // }
    }
}


#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d ef";
        let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "ef"]);
    }
}
