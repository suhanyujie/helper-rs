//! 字符串处理
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// 将分隔符 delimiter 抽象成 D，需要其实现 find_next 方法。
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| c == self).map(|(start, c)| (start, start + c.len_utf8()))
    }
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        // let remainder /* &mut &str */= &mut self.remainder?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}


/// https://www.php.net/strstr
#[derive(Debug)]
struct StrStr<'a> {
    ori_str: &'a str,
    reminder: Option<&'a str>,
}

impl<'a> StrStr<'a> {
    pub fn new(ori: &'a str) ->Self {
        Self{
            ori_str: ori,
            reminder: None,
        }
    }

    pub fn strstr(&self, search: &'a str) -> Option<&'a str> {
        if let Some(posi1) = self.ori_str.find(search) {
            let remain = &self.ori_str[posi1..];
            return Some(remain);
        } else {
            return None;
        }
    }
}


#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn test_strstr() {
        let ori = "hello world...";
        assert_eq!(StrStr::new(ori).strstr("world"), Some("world..."));
    }

    #[test]
    fn it_works() {
        let haystack = "a b c d ef";
        let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "ef"]);
    }

    #[test]
    fn test_split_by_char() {
        let haystack = "a b c d ef e ";
        let letters: Vec<&str> = StrSplit::new(haystack, 'e').collect();
        assert_eq!(letters, vec!["a b c d ", "f ", " "]);
    }

    // 测试字符迭代器的一些方法。
    #[test]
    fn test_char_iter1() {
        let s1 = "hello world";
        // find 之后，会将前面传递过来的值，直接往下传递。
        let res = s1.char_indices().find(|(index, c)| c == &'o').map(|pre_val| pre_val).unwrap_or((1, '0'));
        println!("{:?}", res);
    }
}
