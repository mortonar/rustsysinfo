#![feature(libc)]

extern crate rustsysinfo;

extern crate libc;

use rustsysinfo::sysinfo;
use libc::c_char;
use std::str::from_utf8_unchecked;
use std::ffi::CStr;

pub fn main() {
    match sysinfo() {
        Ok(output) => {
            print_field(output.sysname);
            print_field(output.nodename);
            print_field(output.release);
            print_field(output.version);
            print_field(output.machine);
            print_field(output.__domainname);
        },
        Err(e) => eprintln!("error getting sysinfo: {}", e)
    }
}

fn print_field(field: [c_char; 65usize]) {
    println!("{:?}",to_str(&(&field as *const c_char ) as *const *const c_char));
}

fn to_str<'a>(s: *const *const c_char) -> &'a str {
    unsafe {
        let res = CStr::from_ptr(*s).to_bytes();
        from_utf8_unchecked(res)
    }
}