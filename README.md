# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: using raw pointers to access data after the owning object has been modified or dropped. The code modifies a vector through a raw pointer, then changes the vector. This leads to potential memory corruption or crashes.