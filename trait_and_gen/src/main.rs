use std::fmt::Debug;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

struct Other {
    a: u32,
    b: u32,
}


fn main() {
    println!("Hello, world!");
    let x = Point { x: 5, y: 10 };
    log(&x); // log<Point>(...)
    let y = Other { a: 10, b: 20 };
    // log(&y);
}

fn log<T: Debug>(arg: &T) {
    println!("{:?}", arg);
}

// THIS ISN'T POSSIBLE, because negative trait bounds aren't
// currently a thing (yet), which means we can not have
// specialization, doesn't exist (yet)
// fn log<T: !Debug>(arg: &T)
// {
//     println!("{:?}", arg);
// }

// T: Sized
// Sized: This type has a compile time known size
// Opt out of the Sized restriction like this:
// fn something<T: ?Sized>(arg: T) {
//     // MAKE ROOM TO MOVE arg into this function
// }

// TODO: Explain the one exeception to this rule

// trait Meta: Debug + Display {

// }

trait IntoMyNumber {
    // required methods
    fn into_my_num(&self) -> u32;

    // optional/default methods
    fn hello(&self) {
        println!("Hello! My val is {}", self.into_my_num());
    }

    fn do_special(&self) -> Result<u64, ()> {
        Err(())
    }
}

impl IntoMyNumber for u16 {
    fn into_my_num(&self) -> u32 {
        *self as u32
    }

    fn hello(&self) {
        println!("Hello from u16!");
    }

    fn do_special(&self) -> Result<u64, ()> {
        Ok(*self as u64)
    }
}

trait Centerpoint {
    fn get_center(&self) -> (f64, f64);
}

trait Distance<Other: Centerpoint> {
    fn distance(&self, other: &OtherShape) -> f64;
}

impl Centerpoint for Point {
    fn get_center(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}

impl Distance<OtherShape: Centerpoint> for Point {
    fn distance(&self, other: &OtherShape) -> f64 {
        let our_center = self.get_center();
        let oth_center = other.get_center();
        // do math here
        todo!()
    }
}

// Consumers (users of those capabilities)
// Trait (capability definition)
// Implementors (implementors of those capabilities)


// See you at :22 past the hour!

trait Failable {
    // Associated types in traits
    type Error;
    fn do_something(&mut self) -> Result<u32, Self::Error>;
}

struct Bib {

}

impl Failable for Bib {
    type Error = String;

    fn do_something(&mut self) -> Result<u32, Self::Error> {
        Ok(54)
    }
}

// Marker traits
trait Nothing {}

// Most common ones:
// * Copy - cheap to duplicated
// * Send - Data can be moved/ownership can be
//            passed from one thread to another
// * Sync - Data can be shared between multiple
//            threads at the same time

// #[derive(...)]
// * If all your children impl some trait, then the parent can be derived

#[derive(Debug)]
struct WillDebug {
    a: u32,
    b: String,
    c: Vec<u8>,
}

// Send + Sync are a little special
// If all of your children impl Send, the parent does too.
// If all of your children impl Sync, the parent does too.
// You do not need derive for send/sync.

// Send
// * What could go wrong, sending data from one thread to another
// * What concrete examples do you have
//   (from other langs) of this going wrong?


// Sync
// * What could go wrong, sharing data between N threads
// * What concrete examples do you have
//   (from other langs) of this going wrong?

