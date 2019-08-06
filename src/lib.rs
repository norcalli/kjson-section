#![warn(clippy::all)]

mod byte_section;
mod peek_seek;
mod char_section;

pub use byte_section::ByteSection;
pub use char_section::CharSection;
pub use peek_seek::{PeekSeek, FalliblePeekSeek};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
