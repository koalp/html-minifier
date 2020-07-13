use core::fmt::{self, Formatter};
use core::str::from_utf8_unchecked;

#[inline]
pub(crate) fn str_bytes_fmt(v: &[u8], f: &mut Formatter) -> Result<(), fmt::Error> {
    f.write_fmt(format_args!("{:?}", unsafe { from_utf8_unchecked(v) }))
}

#[inline]
pub(crate) fn is_whitespace(e: u8) -> bool {
    match e {
        0x09..=0x0D | 0x1C..=0x20 => true,
        _ => false,
    }
}

#[inline]
pub(crate) fn is_ascii_control(e: u8) -> bool {
    match e {
        0..=8 | 11..=31 | 127 => true,
        _ => false,
    }
}
