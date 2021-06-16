
1. Redisish protocol parser
    * Write a really simple line parser for a text format
    * SET, GET
    * Match statements, idiomatic string handling
    * Parsing in general (and what can go wrong, and how rust can help)
    * Alternative: Use Serde instead
2. TCP server
    * We implement a loopback server using the std library
    * Single threaded
3. TCP client
    * We implement a loopback client using the std library
    * Single threaded
4. Connected mailbox
    * TCP Server + Redisish parser
    * Single threaded
5. Multithreaded mailbox
    * Take our single threaded connected mailbox
    * Multithreaded (still using the stdlib)
    * Demonstrates sharing data between threads, or sending data between threads
6. Async mailbox
    * A lot like multithreaded example, but using tokio or async-std (at your choice)
