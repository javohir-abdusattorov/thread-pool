## Thread pool implementation

#### Link to the task
Rust book:
https://doc.rust-lang.org/beta/book/ch20-02-multithreaded.html

#### Idea
Firstly implemented for using in web-server, that can respond to multiple requests at once. Limited workers, so that it safe against DoS attacks.
Implemented graceful shutdown so that if **ThreadPool** goes out of scope, it still finishes last jobs - but stops receiving new ones. 
Also implemented error handling and own errors. If any error occurs within dependant modules (```std::mpsc```, ```std::thread```) error will be converted / mapped to **ThreadPoolError** type. This type implemented ```Display``` trait, so it can be printed out

#### Dependencies
- *thread* - standart library implementation of threads
- *Arc* and *Mutex* - to enable locking mechanism between threads, one sender and multiple receivers can work parallel