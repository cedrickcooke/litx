use std;

pub unsafe trait Unslice {
    fn unslice<'a>(&'a self, next: &'a Self) -> Option<&'a Self> {
        if self.is_adjacent(next) {
            unsafe { return Some(self.unslice_unchecked(next)); }
        }
        None
    }

    fn is_adjacent(&self, next: &Self) -> bool;
    unsafe fn unslice_unchecked<'a>(&'a self, next: &'a Self) -> &'a Self;
}

unsafe impl Unslice for str {
    fn is_adjacent(&self, next: &Self) -> bool {
        unsafe { self.as_ptr().offset(self.len() as isize) == next.as_ptr() }
    }

    unsafe fn unslice_unchecked<'a>(&'a self, next: &'a Self) -> &'a Self {
        let slice = std::slice::from_raw_parts(self.as_ptr(), self.len() + next.len());
        std::str::from_utf8_unchecked(slice)
    }
}

unsafe impl <T> Unslice for [T] {
    fn is_adjacent(&self, next: &Self) -> bool {
        unsafe { self.as_ptr().offset(self.len() as isize) == next.as_ptr() }
    }

    unsafe fn unslice_unchecked<'a>(&'a self, next: &'a Self) -> &'a Self {
        std::slice::from_raw_parts(self.as_ptr(), self.len() + next.len())
    }
}

#[cfg(test)]
mod test {
    use super::Unslice;
    const SRC: &'static str = "FOO BAR";

    fn slice() -> (&'static str, &'static str, &'static str) {
        (&SRC[0..3], &SRC[3..4], &SRC[4..7])
    }

    #[test]
    #[should_panic]
    fn unslice_foo_bar() {
        let (foo, spc, bar) = slice();
        foo.unslice(bar).unwrap();
    }

    #[test]
    fn unslice() {
        let (foo, spc, bar) = slice();
        assert!(foo.unslice(spc).unwrap().unslice(bar).unwrap() == SRC);
    }
}
