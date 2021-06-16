use std::thread::{sleep, spawn};
use std::time::Duration;
use std::sync::Mutex;

struct Bar {
    x: u32,
    y: i32,
}

// parking_lot - More kinds of sync primatives outside the
// standard library

fn main() {
    let a: Mutex<u32> = Mutex::new(100);

    if let Ok(hdl) = a.lock() {
        println!("{}", hdl);
    };

    if let Ok(mut hdl) = a.lock() {
        *hdl = 200;
    };

    if let Ok(hdl) = a.lock() {
        println!("{}", hdl);
    };
}


fn a(data: &str) -> String {
    b(data)
}

fn b(data: &str) -> String {
    c(data)
}

fn c(data: &str) -> String {
    // maybe make some modifications here, and give them back
    let foo = String::from("whatever");
    &foo
}

fn whatever() {
    let x = String::from("whatever");
    // Allocate a string on the heap
    // Create a handle to the string on the heap
    // Assign the handle to the binding `x`

} // x falls out of scope here.
  // We can now deallocate x's payload/heap allocation
  // We know this is okay, because if some other reference to
  // x was live, we couldn't have called the destructor here,
  // or we would have gotten a lifetime/borrow checker error

// Generic heap allocation type in Rust is called Box
// Box<u32>
// let a: Box<u32> = Box::new(100);

/* Heap patterns in Rust

* Payload -> Lives on the heap
    * Actual heap allocation
    * Contains the "bulk" of data
* Handle -> Lives on the stack
    * Contains the pointer to the payload
    * Is used to represent ownership
*/
