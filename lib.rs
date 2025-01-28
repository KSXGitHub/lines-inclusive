#![no_std]
#![doc = include_str!("specs.md")]

use core::{iter::FusedIterator, str::SplitInclusive};

mod sealed {
    pub trait Sealed {}
    impl Sealed for str {}
}

/// Extension trait that provides `lines_inclusive` method for `str`.
pub trait LinesInclusive: sealed::Sealed {
    /// Split a string into multiple lines, every line may end with `\n`.
    ///
    /// Note that if a line ends with `\n\r`, the `\r` will be the first character of the next line.
    fn lines_inclusive(&self) -> LinesInclusiveIter<'_>;
}

impl LinesInclusive for str {
    fn lines_inclusive(&self) -> LinesInclusiveIter<'_> {
        LinesInclusiveIter::new(self)
    }
}

/// Iterator over inclusive lines of strings.
///
/// This struct is created by calling [`LinesInclusive::lines_inclusive`].
#[derive(Debug, Clone)]
pub struct LinesInclusiveIter<'a>(SplitInclusive<'a, char>);

impl<'a> LinesInclusiveIter<'a> {
    /// Like [`LinesInclusive::lines_inclusive`].
    fn new(text: &'a str) -> Self {
        LinesInclusiveIter(text.split_inclusive('\n'))
    }
}

impl<'a> Iterator for LinesInclusiveIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl DoubleEndedIterator for LinesInclusiveIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl FusedIterator for LinesInclusiveIter<'_> {}

#[doc = include_str!("README.md")]
mod test_readme {}
