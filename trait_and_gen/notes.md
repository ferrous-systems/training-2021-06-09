

# Send

* What could go wrong, sending data from one thread to another
* What concrete examples do you have
  (from other langs) of this going wrong?

* goroutines
    * data won't be handled as expected
    * Unexpected data sharing
* Huge data: Could be expensive, could be stale

# Sync

* What could go wrong, sharing data between N threads
* What concrete examples do you have
  (from other langs) of this going wrong?

* Unguarded (mutexless access)
* Data livliness (data no longer exists/was relocated)
* Data races: read/write tearing
* Concurrent mutable access
* Perf: False sharing

```c++
void TaskBase::Release() {
    if (m_refCount.decrement() == 0) {
        m_owner->ReleaseTask(this);
        // data race here!
        m_owner = nullptr; // use after free here!
    }
}
```

this was the fix:

```c++
void TaskBase::Release() {
    if (m_refCount.decrement() == 0) {
        auto owner = m_owner;
        m_owner = nullptr;
        owner->ReleaseTask(this);
    }
}
```

* Does this data live long enough
    * Or is the reference to some data valid for long enough
* Who may access this data, mutably or immutably

# Send and the two above concerns

* Live long enough
    * bad: Pointer/Reference "lives long enough"
        * stack data especially notorious for this
    * good: passing (fully) OWNED data (no false sharing)
* Who may access
    * good: If (fully) owned, who may access is clear: NOT the old thread
    * bad: Much less clear if a reference or borrow is involved

```rust
struct ContainsStrRef<'a> {
    foo: &'a str,
}

fn main() {
    let a = String::from("foo");
    let x = ContainsStrRef { foo: &a };
    send(x); // careful about how long `a` lives!
}
```

# Interlude: Why "Lives long enough" is so hard with threads

* Threads live as long as they want.

```rust
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let a = String::from("foo");
    let _ = spawn(|| {
        sleep(Duration::from_secs(3));
        println!("{}", a);
    });
    sleep(Duration::from_secs(1));
} // a is destroyed
```

`'static`: Does (or could potentially, if needed to) live "forever"

# Sync and the two above concerns

* Live long enough
    * For not totally owned data, how long are references valid for?
    * If two threads can access data for as long as they like, who
        runs the destructor? And when?
    * Do we know if it exists yet?
        * Rust doesn't have null
        * miiiight not be a thing in rust, let me know if you disagree
* Who may access
    * Reading/Writing
    * If everyone can ONLY read: Okay!
    * If ONLY one writer AND zero readers: Okay (but this
        defeats the purpose of sync, which is sharing)
        * Very similar to our "shared XOR mutable"
    * What happens if we want mixed access (read and write)?
        * We need some way to handle this

# Interlude: Global variables in Rust

Rust DOES have global variables.

They are NOT super idiomatic, and generally, harder to use than you might think.

```rust
use std::thread::{sleep, spawn};
use std::time::Duration;

struct Bar {
    x: u32,
    y: i32,
}

// global variable (immutable)
static A: u32 = 100;
static B: Bar = Bar { x: 10, y: 20 };

// global variable (mutable)
// PRETTY MUCH ALWAYS A FOOTGUN
// PRETTY MUCH NEVER USE THEM EVER
// THERE ARE BETTER WAYS TO DO THIS
static mut M: Bar = Bar { x: 10, y: 20 };

fn main() {
    println!("{}", A);

    let aref: &'static u32 = &A;

    // impossible, unless B: Copy
    foo(B);

    println!("{}", M); // won't compile

    // unsafe means: "Manual mode"
    // RULES STILL APPLY
    // But, the programmer must enforce them, NOT the compiler
    unsafe {
        let a = &mut M;
        M = Bar { x: 20, y: 30 }; // STILL undefined behavior
        println!("{}", a);
    }

    // if B: Clone
    foo(B.clone());

    let _ = spawn(|| {
        println!("{}", A);
    });
}

fn foo(data: Bar) {

}
```

* Who owns this data?
    * Who runs the destructor, and when?
    * No one!
* using mutably: PROBLEM
    * Almost universally: You want "inner mutability"

* Guesses:
    * Owners: Main thread by default?
    * Location: `.data` segment?
        * `.data` OR `.text`, depends. `.rodata`
        * the depends is "inner mutability", more on that soon

* Before `main()` is called
    * operating system will do some stuff
    * runtime will do some stuff
        * C: `crt0.o` (usually provided by your compiler)
        * Rust: functionally equivalent to `crt0.o`
* during `main()`'s execution
    * When we talk about "forever" in rust, that's just here.
* after `main()` returns
    * tear down steps by the runtime (often: nothing)
    * tear down steps by the operating system

* Instead of mutable statics:
    * If you need "lazy loaded data" or "singletons", but are still
        read only: Consider `lazy_static` or `OnceCell`
    * If you need truly mutable globals: consider inner mutability
        * (think Mutex)


```rust
use std::thread::{sleep, spawn};
use std::time::Duration;
use std::sync::Mutex;

struct Bar {
    x: u32,
    y: i32,
}

static A: u32 = Mutex::new(100);
// static B: Bar = Bar { x: 10, y: 20 };

fn main() {
    if let Ok(hdl) = A.lock() {
        println!("{}", hdl);
    }
}

```
