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

A string consists of -> in memory, information stored on the stack
- pointer to the memory adress i.e. index zero of the string
- the length i.e. how much memory is stored for the content
- the capacity i.e. amount of memory received from the allocator
= means we the created string variable is assigned to another, these infos get copied, but not its "acutal" content inside the memory.

The 'double free error' problem
=> When e.g. a string is assigned to another string and they go out of scope, it is normally trying to clear up memory twice (because we have to 'two referencs', SO

Rust does this: If we assign one string to another and try to use the first one, it doesnt work because rust considers tring one as 'no longer valid' to prevent this error

-> When Rust does invalidate the first string, it is considered a MOVE
-> If we want to deep copy the data on the heap, it is called CLONING

### Stack-Only Data: copy

- > Further infos in 4.1

### Ownership and functions
