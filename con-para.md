

* Concurrency
    * Waiting on many things at the same time
    * Sockets - nginx
    * "juggle more things at once" => speed us up
* Parallelism
    * Working on many things at the same time
    * Mandelbrot
    * Ray Tracing
    * video encoding, rendering
    * Pretty hard upper limit, usually resource bound for this

* Rust has no opinions on concurrency vs parallelism
* concurrency and parallelism are not necessarily BETTER than eachother
    * Different tools for different jobs
* ALL of rusts safety and correctness guarantees apply EQUALLY, regardless of C vs P


* Tools like tokio, async-std, smol, all these things: These are Concurrency tools
* Tools like std::thread, and other libraries like Rayon: These are Parallelism tools

# Parallelism

* Rust's std library has built-in support for OS-native threads
* Data can be (carefully) sent or shared between threads
* Unit of work is typically a "thread"
* The "engine" that makes this work, is your operating system

* "pre-emptive multitasking"

# Concurrency

* Rust has some language level features for doing concurrency
    * Futures, `async/await` syntax, a handful of things in the std library
* Data can be (carefully) sent or shared between Tasks
    * Rust doesn't require different concurrent tasks to be on the same or different threads
    * Tasks MAY move between threads (sometimes, and carefully)
* Unit of work, is typically a "task", or a "future"
    * Tasks are a sequence of pausable control flow
    * Futures
* The "engine" that makes this work is called an "Executor" (tokio, async-std)
    * Executors are responsible for "juggling" tasks and futures

* "cooperative multitasking"

```rust
fn wait_forever() {
    loop { }
}
```

```rust
async fn not_forever() -> Result<(), ()> {
    let mut x = socket().await?;
    let foo = &x.wait_for_data("hello").await?;
    x.send_data_back("goodbye").await?;
    println!("{}", foo);
    x.close().await
}
```

```rust
fn not_forever() -> Result<(), ()> {
    let mut x = loop {
        if let Ok(s) = socket() {
            break s;
        } else {
            thread.sleep(Duration::from_millis(100));
        }
    };

    x.wait_for_data("hello").await?;
    x.send_data_back("goodbye").await?;
    x.close().await
}
```
