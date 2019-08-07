use crate::PeekSeek;

#[derive(Debug)]
pub struct ByteSection<'a> {
    pub n: usize,
    pub src: &'a [u8],
}

use std::fmt;

impl<'a> fmt::Display for ByteSection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ByteSection(n={}, ", self.n)?;
        if let Some(c) = self.peek() {
            if c.is_ascii_graphic() {
                write!(f, "head={}/'{}', ", c, c as char)?;
            } else {
                write!(f, "head={}, ", c)?;
            }
        } else {
            write!(f, "head=None, ")?;
        }
        write!(f, "s={:?})", self.src)?;
        Ok(())
    }
}

// TODO make this a trait and implement for "rope" like structure.
// aka discontinuous strings.

// struct ByteSubSection<'a> {
//     parent: &'a mut ByteSection<'a>,
// }

// impl<'a> ByteSubSection<'a> {
//     pub
// }

impl<'a> ByteSection<'a> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> ByteSection<'a> {
        ByteSection { n: 0, src: buf }
    }

    #[inline]
    pub fn take(&mut self, n: usize) -> &'a [u8] {
        let result = &self.src[self.n..self.src.len().min(self.n + n)];
        self.n += result.len();
        result
    }

    #[inline]
    pub fn slice_to_end(&self) -> &'a [u8] {
        &self.src[self.n..]
    }

    #[inline]
    pub fn slice_from_start(&self) -> &'a [u8] {
        &self.src[..self.n]
    }

    /// Skip until `f` is satisfied, but don't consume the byte at `f`.
    #[inline]
    fn skip_until_fallback(&mut self, target: u8) -> usize {
        let n = self.n;
        let maxn = self.src.len();
        let src = self.src;
        while self.n < maxn {
            if src[self.n] == target {
                return self.n;
            }
            self.n += 1;
        }
        self.n - n
    }

    /// Skip until `f` is satisfied, but don't consume the byte at `f`.
    #[inline]
    #[cfg(all(
        target_feature = "avx2",
        any(target_arch = "x86", target_arch = "x86_64")
    ))]
    unsafe fn skip_until_avx2(&mut self, target: u8) -> usize {
        self.skip_until_fallback(target)
    }
}

impl<'a, __IdxT> ::std::ops::Index<__IdxT> for ByteSection<'a>
where
    __IdxT: std::slice::SliceIndex<[u8]>,
{
    type Output = <[u8] as ::std::ops::Index<__IdxT>>::Output;

    #[inline]
    fn index(&self, idx: __IdxT) -> &Self::Output {
        self.src.index(idx)
    }
}

impl crate::PeekSeek for ByteSection<'_> {
    type Item = u8;

    #[inline]
    fn peek(&self) -> Option<u8> {
        self.src.get(self.n).copied()
    }

    #[inline]
    fn next(&mut self) -> Option<u8> {
        if let Some(c) = self.peek() {
            self.n += 1;
            Some(c)
        } else {
            None
        }
    }

    #[inline]
    fn skip(&mut self, n: usize) -> usize {
        self.n = self.src.len().min(n + self.n);
        self.n
    }

    /// Skip until `f` is satisfied, but don't consume the byte at `f`.
    #[inline]
    fn skip_until_pattern<F: Fn(u8) -> bool>(&mut self, f: F) -> usize {
        let n = self.n;
        if let Some(i) = self.src.iter().skip(self.n).copied().position(f) {
            self.n += i;
        } else {
            self.n = self.src.len();
        }
        self.n - n
    }

    // /// Skip until `f` is satisfied, but don't consume the byte at `f`.
    // #[inline]
    // #[cfg(all(
    //     target_feature = "avx2",
    //     any(target_arch = "x86", target_arch = "x86_64")
    // ))]
    // fn skip_until(&mut self, target: Self::Item) -> usize {
    //     // _mm_cmpeq_epi8_mask
    //     // if is_x86_feature_detected!("avx2") {
    //     use std::arch::x86_64::{
    //         __mm256i, _blsmsk_u64, _mm256_cmpeq_epi8, _mm256_movemask_epi8, _mm256_set1_epi8,
    //     };
    //     unsafe {
    //         let mm_target: __mm256i = _mm256_set1_epi8(target);
    //         let mut n = self.n;
    //         let maxn = self.src.len() - (self.src.len() - n) % 32;
    //         let src = self.src.as_ptr();
    //         while n < maxn {
    //             let input = _mm256_loadu_si256(src + n);
    //             let mask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(mm_target, data));
    //             let parts: [u64; 4] = std::mem::transmute(mask);
    //             let mut x = 0;
    //             for i in 0..4 {
    //                 x += _popcnt64(_blsmsk_u64(parts[i]));
    //             }
    //             n += x;
    //             if x < 32 {
    //                 break;
    //             }
    //         }
    //     }
    //     self.skip_until_pattern(|c| c == target)
    // }

    /// Skip until `f` is satisfied, but don't consume the byte at `f`.
    #[inline]
    #[cfg(all(
        target_feature = "avx2",
        any(target_arch = "x86", target_arch = "x86_64")
    ))]
    fn skip_until(&mut self, target: Self::Item) -> usize {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.skip_until_avx2(target) };
            }
        }
        self.skip_until_fallback(target)
    }

    // /// Skip until `f` is satisfied, but don't consume the byte at `f`.
    // #[inline]
    // fn skip_until(&mut self, target: Self::Item) -> usize {
    //     self.skip_until_pattern(|c| c == target)
    // }

    #[inline]
    fn is_empty(&self) -> bool {
        self.n == self.src.len()
    }
}

// macro_rules! fallible_peek_seek {
//     (eof = $eof:expr; unexpected = $u:expr; $value:ty) => (
//         #[derive(crate::derive_deref::DerefMut, crate::derive_deref::Deref)]
//         struct $
//     )
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_section_test() {
        let input = "hello world";
        let mut s = ByteSection::new(input.as_bytes());
        assert_eq!(&s.src[s.n..], s.slice_to_end());
        assert_eq!(&s.src[s.n..], &s[s.n..]);
        assert_eq!(&s[s.n..], input.as_bytes());
        assert_eq!(s.peek(), Some(b'h'));
        assert_eq!(s.next(), Some(b'h'));
        assert_eq!(s.n, 1);
        assert_eq!(&s.src[..s.n], b"h");
        for _ in 0..4 {
            s.next();
        }
        assert_eq!(s.peek(), Some(b' '));
        for _ in 0..10 {
            s.next();
        }
        assert_eq!(s.peek(), None);
        assert_eq!(s.n, input.len());
        assert_eq!(&s.src[..s.n], input.as_bytes());
        assert_eq!(s.src, input.as_bytes());
        assert_eq!(&s.src[s.n..], b"");
    }
}
