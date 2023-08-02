### OWNERSHIP 

## Ownership is a set of rules that govern how a Rust program manages memory.
##  Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.


## The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating 

## Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.

## Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.

#### Ownership Rules

## First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:
# Each value in Rust has an owner.
# There can only be one owner at a time.
# When the owner goes out of scope, the value will be dropped.