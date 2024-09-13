extern crate libc;
use libc::{c_int, c_void};

extern "C" {
    pub fn lzo1z_compress(
        src: *const u8,
        src_len: libc::size_t,
        dst: *mut u8,
        dst_len: *mut libc::size_t,
        work_mem: *mut c_void,
    ) -> c_int;

    pub fn lzo1z_decompress(
        src: *const u8,
        src_len: libc::size_t,
        dst: *mut u8,
        dst_len: *mut libc::size_t,
        work_mem: *mut c_void,
    ) -> c_int;
}

use libc::size_t;

pub fn compress(input: &[u8], output: &mut [u8]) -> Result<usize, String> {
    let mut output_len = output.len() as size_t;
    let mut work_mem = vec![0; 4096]; // Adjust size if needed

    let res = unsafe {
        lzo1z_compress(
            input.as_ptr(),
            input.len() as size_t,
            output.as_mut_ptr(),
            &mut output_len,
            work_mem.as_mut_ptr() as *mut _,
        )
    };

    if res == 0 {
        Ok(output_len as usize)
    } else {
        Err("Compression failed".to_string())
    }
}

pub fn decompress(input: &[u8], output: &mut [u8]) -> Result<usize, String> {
    let mut output_len = output.len() as size_t;
    let mut work_mem = vec![0; 4096]; // Adjust size if needed

    let res = unsafe {
        lzo1z_decompress(
            input.as_ptr(),
            input.len() as size_t,
            output.as_mut_ptr(),
            &mut output_len,
            work_mem.as_mut_ptr() as *mut _,
        )
    };

    if res == 0 {
        Ok(output_len as usize)
    } else {
        Err("Decompression failed".to_string())
    }
}