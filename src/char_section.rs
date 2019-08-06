#![warn(clippy::all)]

use std::iter::Peekable;
use std::str::Chars;

use crate::PeekSeek;

// use unicode_segmentation::UnicodeSegmentation;

// TODO make this a trait and implement for "rope" like structure.
// aka discontinuous strings.

#[derive(Clone, Debug)]
pub struct CharSection<'a> {
    n: usize,
    s: &'a str,
    chars: Peekable<Chars<'a>>,
    head: Option<char>,
}

impl<'a> CharSection<'a> {
    #[inline]
    pub fn new(s: &'a str) -> Self {
        let mut chars = s.chars().peekable();
        Self {
            s,
            n: 0,
            head: chars.peek().copied(),
            chars,
        }
    }

    // #[inline]
    // pub fn new_with_offset(s: &'a str, n: usize) -> Self {
    //     let s = &s[n..];
    //     Self {
    //         s,
    //         n,
    //         chars: s.chars().peekable(),
    //     }
    // }

    #[inline]
    #[cfg(test)]
    fn after(&self) -> &'a str {
        &self.s[self.n..]
    }
}

impl<'a> PeekSeek for CharSection<'a> {
    type Item = char;

    #[inline]
    fn peek(&self) -> Option<char> {
        self.head
    }

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.chars.next().map(|c| {
            self.n += c.len_utf8();
            self.head = self.chars.peek().copied();
            c
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_section_test() {
        let input = "hello world";
        let mut s = CharSection::new(input);
        assert_eq!(s.after(), input);
        assert_eq!(s.peek(), Some('h'));
        assert_eq!(s.next(), Some('h'));
        assert_eq!(s.n, 1);
        assert_eq!(&s.s[..s.n], "h");
        for _ in 0..4 {
            s.next();
        }
        assert_eq!(s.peek(), Some(' '));
        for _ in 0..10 {
            s.next();
        }
        assert_eq!(s.peek(), None);
        assert_eq!(s.n, input.len());
        assert_eq!(&s.s[..s.n], input);
        assert_eq!(s.after(), "");
    }
}
