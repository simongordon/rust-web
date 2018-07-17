use std::os::raw::c_char;
use std::collections::HashMap;
use std::ffi::CString;


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
}
