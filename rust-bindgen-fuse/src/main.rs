#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::env::args;
use std::env::args_os;
use std::ffi::CString;
use std::mem::size_of;
use std::os::unix::ffi::OsStrExt;
use std::os::raw::c_int;

fn main() {
    let ops: fuse_operations = fuse_operations {
        getattr: None,
        readlink: None,
        mknod: None,
        mkdir: None,
        unlink: None,
        rmdir: None,
        symlink: None,
        rename: None,
        link: None,
        chmod: None,
        chown: None,
        truncate: None,
        open: None,
        read: None,
        write: None,
        statfs: None,
        flush: None,
        release: None,
        fsync: None,
        setxattr: None,
        getxattr: None,
        listxattr: None,
        removexattr: None,
        opendir: None,
        readdir: None,
        releasedir: None,
        fsyncdir: None,
        init: None,
        destroy: None,
        access: None,
        create: None,
        lock: None,
        utimens: None,
        bmap: None,
        ioctl: None,
        poll: None,
        write_buf: None,
        read_buf: None,
        flock: None,
        fallocate: None,
        copy_file_range: None,
    };

    let argc: i32 = args_os().len() as i32;
    let args: Vec<CString> = args_os()
        .into_iter()
        .map(|arg| {
            arg.to_str()
                .and_then(|s| {
                    CString::new(s)
                        .map(|c_string| {
                            dbg!(&c_string);
                            c_string
                        })
                        .ok()
                })
                .expect("Expected valid arg input")
        })
        .collect();

    let mut argv: Vec<*const ::std::os::raw::c_char> =
        args.iter().map(|arg| arg.as_ptr()).collect();

    unsafe {
        fuse_main_real(
            argc,
            argv.as_mut_ptr() as *mut *mut ::std::os::raw::c_char,
            &ops,
            size_of::<fuse_operations>(),
            std::ptr::null_mut(),
        );
    }
}