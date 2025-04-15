1) I saw someone's code fail to compile because they 
were trying to send non-thread-safe data across threads. 
How does the Rust language allow for static (i.e. at compile time)
guarantees that specific data can be sent/shared acrosss threads?

- Rust employs marker traits Send and Sync that encode thread-safety in the type system. Send guarantees safe ownership transfer between threads. The compiler statically verifies these traits during compilation, rejecting any code that would violate thread-safety guarantees. This zero-cost abstraction prevents data races without runtime overhead.

2) Do you have to then implement the Send and Sync traits for 
every piece of data (i.e. a struct) you want to share and send across threads?

- No. Rust auto-derives these traits structurally—if all components are Send/Sync, the composite type inherits those traits. However, this isn't implicit implementation but type-based inference. Only types with "interior unsafety" require manual unsafe impl declarations, where you assume responsibility for guarantees the compiler cannot verify.

3) What types in the course have I seen that aren't Send? Give one example, 
and explain why that type isn't Send 

- Rc<T> isn't Send because its reference counting is non-atomic. If shared across threads, concurrent increments/decrements would race, potentially resulting in use-after-free vulnerabilities when the count erroneously reaches zero. Other non-Send types include RefCell<T>, raw pointers, and thread-local types where cross-thread movement violates their fundamental invariants.

4) What is the relationship between Send and Sync? Does this relate
to Rust's Ownership system somehow?

- Formally: Type T is Sync if and only if &T is Send. They extend Rust's ownership model to multithreaded contexts: Send governs ownership transfer (moves) across threads, while Sync governs reference sharing. Together they statically encode the thread-safety subset of Rust's broader memory safety guarantees.

5) Are there any types that could be Send but NOT Sync? Is that even possible?

- Yes, notably types with unsynchronized interior mutability. Cell<T> is Send because transferring ownership is safe, but not Sync because its methods modify contents through &self, causing data races if shared. RefCell<T> similarly lacks atomic borrow counters. These types demonstrate that safe transferability doesn't imply safe shareability.

6) Could we implement Send ourselves using safe rust? why/why not?

- No. Implementing Send requires unsafe impl because you're asserting thread-safety guarantees the compiler cannot verify. Only the type author can determine if internal state can safely transfer across thread boundaries. This is precisely what unsafe is designed for—taking responsibility for invariants beyond the compiler's verification capabilities.
