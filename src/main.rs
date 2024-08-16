#![no_std]
#![no_main]

use core::str;
use libc_print::libc_println;
use once_cell::sync::Lazy;

static MESSAGE: Lazy<[u8; 32]> = Lazy::new(|| {
    let original = b"This is better than lazy static!";
    let mut buffer = [0u8; 32];

    for (i, &b) in original.iter().enumerate() {
        buffer[i] = b.to_ascii_uppercase()
    }

    buffer
});

#[no_mangle]
pub extern "C" fn main() {
    let message_str = str::from_utf8(&*MESSAGE).unwrap_or_default();
    libc_println!("{}", message_str);
}
