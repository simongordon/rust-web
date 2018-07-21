// extern crate libc;

use std::os::raw::c_char;
use std::collections::HashMap;
use std::ffi::CString;


// mod externs;



extern {
    fn alert(ptr: *const u8, number: u32);
}

#[no_mangle]
pub extern "C" fn run() {
    unsafe {
        let x = b"Hello World!\0";
        alert(x as *const u8, 42);
    }
}


#[no_mangle]
pub fn get_html() -> *mut c_char {
    CString::new("<h1>Hi!</h1><p>This is generated content.</p>").unwrap().into_raw()
}

#[no_mangle]
pub fn get_data() -> *mut c_char {
    let mut data = HashMap::new();
    data.insert("Alice", "send");
    data.insert("Bob", "recieve");
    data.insert("Carol", "intercept");

    let descriptions = data.iter()
        .map(|(p,a)| format!("{} likes to {} messages", p, a))
        .collect::<Vec<_>>();

    CString::new(descriptions.join(", "))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");

    // Calls JS alert
    // externs::alert("Hello, alert!");
    // alert("Hello, alert!");
    //     unsafe {
    //     let x = b"Hello World!\0";
    //     alert(x as *const u8, 42);
    // }
    // Calls JS eval
    // externs::eval("console.log('Hello, eval!')");
// Redirects to JS console.log
}
