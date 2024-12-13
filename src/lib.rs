use std::fs::File;
use std::io::{BufReader, StdoutLock, Write};
use std::{env, fs, ptr};

#[inline]
pub fn get_input_reader() -> BufReader<File> {
    unsafe { BufReader::new(File::open(env::args().nth(1).unwrap_unchecked()).unwrap_unchecked()) }
}

#[inline]
pub fn get_input_as_string() -> String {
    unsafe {
        fs::read_to_string(env::args().nth(1).unwrap_unchecked()).unwrap_unchecked()
    }
}

#[inline]
pub fn print_answer(part1: u32, part2: u32) {
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();

    fast_print_u32(&mut stdout_lock, part1);
    unsafe { stdout_lock.write_all(b"\n").unwrap_unchecked(); }
    fast_print_u32(&mut stdout_lock, part2);
}

#[inline]
fn fast_print_u32(stdout_lock: &mut StdoutLock, mut n: u32) {
    const DIGITS_LUT: &[u8; 200] = b"0001020304050607080910111213141516171819\
                                     2021222324252627282930313233343536373839\
                                     4041424344454647484950515253545556575859\
                                     6061626364656667686970717273747576777879\
                                     8081828384858687888990919293949596979899";

    // Buffer of max 10 bytes for storing digits of a u32
    let mut buffer = [0u8; 10];
    let mut i = buffer.len();

    // Process digits two at a time for fast handling of larger numbers
    while n >= 100 {
        let rem = (n % 100) as usize;
        n /= 100;
        i -= 2;
        unsafe {
            ptr::copy_nonoverlapping(
                DIGITS_LUT.as_ptr().add(rem * 2),
                buffer.as_mut_ptr().add(i),
                2,
            );
        }
    }

    // Handle the remaining one or two digits (0-99)
    if n < 10 {
        i -= 1;
        buffer[i] = b'0' + n as u8;
    } else {
        let rem = (n % 100) as usize;
        i -= 2;
        unsafe {
            ptr::copy_nonoverlapping(
                DIGITS_LUT.as_ptr().add(rem * 2),
                buffer.as_mut_ptr().add(i),
                2,
            );
        }
    }

    // Write only the filled portion of the buffer
    unsafe { stdout_lock.write_all(&buffer[i..]).unwrap_unchecked(); }
}
