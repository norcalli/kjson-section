// pub trait PeekSeek: Clone + Sized {
pub trait PeekSeek: Sized {
    type Item: Copy + PartialEq + Eq;

    // fn peek(&self) -> Option<&Self::Item>;
    fn peek(&self) -> Option<Self::Item>;

    fn next(&mut self) -> Option<Self::Item>;

    #[inline]
    fn check_next(&mut self, target: Self::Item) -> bool {
        if self.peek() == Some(target) {
            self.next();
            true
        } else {
            false
        }
    }

    #[inline]
    fn check_next_pattern<F: Fn(Self::Item) -> bool>(&mut self, f: F) -> bool {
        match self.peek() {
            Some(c) if f(c) => {
                self.next();
                true
            }
            _ => false,
        }
    }

    #[inline]
    fn peek_next_pattern<F: FnOnce(Self::Item) -> bool>(&mut self, f: F) -> bool {
        self.peek().map(f).unwrap_or(false)
    }

    #[inline]
    fn peek_next(&mut self, target: Self::Item) -> bool {
        self.peek() == Some(target)
    }

    #[inline]
    fn skip(&mut self, n: usize) -> usize {
        for i in 0..n {
            if self.next().is_none() {
                return i;
            }
        }
        n
    }

    /// Skip until `f` is satisfied, but don't consume the byte at `f`.
    #[inline]
    fn skip_until<F: Fn(Self::Item) -> bool>(&mut self, f: F) -> usize {
        let mut n = 0;
        while let Some(c) = self.peek() {
            if f(c) {
                break;
            }
            n += 1;
            self.next();
        }
        n
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.peek().is_none()
    }
}

pub trait FalliblePeekSeek: PeekSeek {
    type Error;

    fn eof_error(&self) -> Self::Error;
    fn unexpected_error(&self, c: Self::Item) -> Self::Error;

    // #[inline]
    // fn expect_next(&mut self, target: Self::Item) -> std::result::Result<(), Self::Error> {
    //     match self.next() {
    //         Some(c) if c == target => Ok(()),
    //         Some(c) => Err(self.unexpected_error(c)),
    //         None => Err(self.eof_error()),
    //     }
    // }

    #[inline]
    fn expect_next(&mut self, target: Self::Item) -> std::result::Result<Self::Item, Self::Error> {
        self.expect_next_pattern(|c| c == target)
    }

    #[inline]
    fn expect(&mut self) -> std::result::Result<Self::Item, Self::Error> {
        self.next().ok_or_else(|| self.eof_error())
    }

    #[inline]
    fn expect_next_pattern<F: Fn(Self::Item) -> bool>(
        &mut self,
        f: F,
    ) -> std::result::Result<Self::Item, Self::Error> {
        match self.next() {
            Some(c) if f(c) => Ok(c),
            Some(c) => Err(self.unexpected_error(c)),
            None => Err(self.eof_error()),
        }
    }
}
