struct Zst;


fn main() {
    println!("Hello, world!");
}


// in library A
fn get_token(&mut self) -> Option<Zst> {
    // if socket is open, return a Zst
    Some(Zst)
}

fn access_file(&mut self, zst: Zst) {
    // ... access the file
}

// in library B
if let Some(token) = foo.get_token() {
    foo.access_file(token);
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn submit_work_request(&mut self, req: Request) -> Result<(), WorkError> {
    todo!()
}

// Clone: deepcopy
// Copy: copy/shallow copy/memcpy

fn foo() {
    let my_arr: [i32; 50] = [42; 50];
    let my_str_arr: [String; 50] = [String::from("hello"); 50];

    let a = &my_arr[0..10];
    let b = &my_arr[0..20];
    let c = &my_arr[10..20];

    let foo = String::from("Hello");

    // Force it to be dropped here
    drop(foo); // A func that causes a value to fall out of scope

    // Basically never what you want
    foo.drop(); // a trait interface that the compiler will call
                // automatically for you exactly when necessary

    let x = 42;

    drop(x); // CLONE OF X will be dropped here
} // The "REAL" X will be dropped here
