use core::str;

pub trait Read<'de> {
    fn next(&mut self) -> Option<u8>;
    fn peek(&mut self) -> Option<u8>;
    fn discard(&mut self);
    fn byte_offset(&self) -> usize;
}

pub struct SliceRead<'a> {
    slice: &'a [u8],
    index: usize,
}

pub struct StrRead<'a> {
    delegate: SliceRead<'a>,
}

impl<'a> SliceRead<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        SliceRead { slice, index: 0 }
    }
}

impl<'a> Read<'a> for SliceRead<'a> {
    #[inline]
    fn next(&mut self) -> Option<u8> {
        if self.index < self.slice.len() {
            let ch = self.slice[self.index];
            self.index += 1;
            Some(ch)
        } else {
            None
        }
    }

    #[inline]
    fn peek(&mut self) -> Option<u8> {
        if self.index < self.slice.len() {
            Some(self.slice[self.index])
        } else {
            None
        }
    }

    #[inline]
    fn discard(&mut self) {
        self.index += 1;
    }

    fn byte_offset(&self) -> usize {
        self.index
    }
}

impl<'a> StrRead<'a> {
    pub fn new(s: &'a str) -> Self {
        StrRead {
            delegate: SliceRead::new(s.as_bytes()),
        }
    }
}

impl<'a> Read<'a> for StrRead<'a> {
    #[inline]
    fn next(&mut self) -> Option<u8> {
        self.delegate.next()
    }

    #[inline]
    fn peek(&mut self) -> Option<u8> {
        self.delegate.peek()
    }

    #[inline]
    fn discard(&mut self) {
        self.delegate.discard();
    }

    fn byte_offset(&self) -> usize {
        self.delegate.byte_offset()
    }
}
