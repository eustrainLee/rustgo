use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn plus(a:u64,b:u64) -> u64;
    
    fn get(index:u64) -> *const c_char;

    fn start(str:*const c_char) -> u64;
}

pub fn plus_(a:u64, b:u64) -> u64 {
    let result = unsafe { 
        plus(a, b)
    };
    result
}


pub fn get_(index:u64) -> String {
    let result = unsafe { 
        get(index)
    };
    let c_str = unsafe { CStr::from_ptr(result) };
    let string = c_str.to_str().expect("Error translating SQIP from library");
    string.to_string()
}

pub fn start_(name:String) -> u64 {
    let c_str: CString = CString::new(name).expect("CString::new failed");
    let c_world = c_str.as_ptr();
    let result = unsafe { 
        start(c_world)
    };
    result
}