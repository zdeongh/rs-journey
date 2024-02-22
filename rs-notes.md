# RS-Notes

## Ownership

Set of rules (compiler-checked and non-compilalbe if errors) that govern how a Rust program manages memory.

### Throwback -> Stack/ Heap

Stack -> Last in, first out
adding data to stack: push
removing data from the stack: pop
=> all data must have a known and fixed size
Data with unknown size must be be stored on the heap

Heap -> Put data on it, certain amount of space is requested
Allocating -> Requesting data and get a pointer to the reserved memory

=> Pushing on the stack is faster than allocating memory

- Ownernship does: Keeps track what data is on the heap and clean up duplicates on the heap

### Ownership rules

- Each value has an Owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Memory and Allocation

- String type: mutable, growable piece of text, that its size unknown at compile time
means:

- The memory must be requested from the memory allocator at compile-time
- Way is needed to return this memory to the allocator when finished

First part: Memory is requested by me when calling String::from("hello")
