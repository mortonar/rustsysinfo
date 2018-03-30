#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::io::{Error,ErrorKind,Result};
use std::process::Command;
use std::process::Output;
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

static NOTHING: c_char = 0;

pub fn sysinfo() -> Result<utsname> {
    let mut uname_status = -1;
    let mut utsname_info = utsname {
        sysname: [NOTHING; 65],
        nodename: [NOTHING; 65],
        release: [NOTHING; 65],
        version: [NOTHING; 65],
        machine: [NOTHING; 65],
        __domainname: [NOTHING; 65]
    };
    unsafe {
        uname_status = uname(&mut utsname_info);
    }
    match uname_status {
        0 => Ok(utsname_info),
        _ => Err(Error::new(ErrorKind::Other, ""))
    }
}
