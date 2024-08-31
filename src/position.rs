// This module is taken from github.com/Darksecond/lox
// as I was looking for a better way to handle diagnostics in my project
// without the combinatorial explosion of custom nested error types that
// I was headed towards in previous approach.
//
// yoink

/// TypeSafe u32 wrapper with some helpful methods for handling iterating over character's
/// positions which may or may not be valid ASCII as expected in lox.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct BytePos(pub u32);

impl BytePos {
    pub fn shift(self, c: char) -> Self {
        BytePos(self.0 + c.len_utf8() as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Span {
    pub start: BytePos,
    pub end: BytePos,
}

/// Simple Error Diagnostics Wrapper.
///
/// Contains a message and a Span (starting and ending BytePos) of the
/// offending token.
pub struct Diagnostic {
    pub span: Span,
    pub message: String,
}

impl Diagnostic {
    pub fn new<T>(message: T, start: BytePos, end: BytePos) -> Self
    where
        T: Into<String>,
    {
        Self {
            message: message.into(),
            span: Span { start, end },
        }
    }
}

/// Wrapper for various types within the interpreter. Allows for keeping the starting
/// and ending BytePosition of the value.
pub struct WithSpan<T> {
    pub value: T,
    pub span: Span,
}

impl<T> WithSpan<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    pub const fn empty(value: T) -> Self {
        Self {
            value,
            span: Span {
                start: BytePos(0),
                end: BytePos(0),
            },
        }
    }

    /// Extract value from WithSpan
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl Span {
    pub const fn empty() -> Self {
        Self {
            start: BytePos(0),
            end: BytePos(0),
        }
    }

    pub fn union_span(a: Self, b: Self) -> Self {
        Self {
            start: std::cmp::min(a.start, b.start),
            end: std::cmp::max(a.end, b.end),
        }
    }

    pub fn union<A, B>(a: &WithSpan<A>, b: &WithSpan<B>) -> Self {
        Self::union_span(a.into(), b.into())
    }
}

impl<T> From<WithSpan<T>> for Span {
    fn from(value: WithSpan<T>) -> Self {
        value.span
    }
}

impl<T> From<&WithSpan<T>> for Span {
    fn from(value: &WithSpan<T>) -> Span {
        value.span
    }
}

pub struct LineOffsets {
    offsets: Vec<u32>,
    len: u32,
}

impl LineOffsets {
    pub fn new(data: &str) -> Self {
        let mut offsets = vec![0];
        let len = data.len() as u32;

        for (i, val) in data.bytes().enumerate() {
            if val == b'\n' {
                offsets.push((i + 1) as u32);
            }
        }

        Self { offsets, len }
    }

    /// Finds the line number of a BytePos in the
    /// source data.
    ///
    /// Panics if the given byte position exceeds the length
    /// of the input data.
    pub fn line(&self, pos: BytePos) -> usize {
        let offset = pos.0;
        assert!(offset <= self.len);

        // binary search is used here as the Err path (element not found) returns
        // a valid index at which the element could have been found in the sorted
        // array. Since we only store the offsets of the \n chars, this in effect
        // gives us an n log n method to find the closest preceding newline for
        // any given bytepos.
        match self.offsets.binary_search(&offset) {
            Ok(line) => line,
            Err(line) => line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BytePos, LineOffsets};

    #[test]
    fn test_offset_gives_correct_line() {
        let of = LineOffsets::new("line1\nline2\nline3\n");
        let res = of.line(BytePos(8));

        assert_eq!(res, 2);
    }
}
