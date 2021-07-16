# Stack Data Structure

A stack is a linear data structure that follows the principle of **LIFO**, the last element inserted inside the stack is removed first.

## LIFO principle of stack
Putting an item on top of the stack is called **push** and removing an item is called **pop**.

## Basic operations of stacks
There are some basic operations that allow us to perform different actions on a stack:
- **Push**: Add an element to the top of a stack.
- **Pop**: Remove an element from the top of a stack.
- **IsEmpty**: Check if the stack is empty.
- **IsFull**: Check if the stack is full.
_ **Peek**: Get the value of the top element without removing it.

## Working of Stack Data structure
The operation work as follows:
1. A pointer called `TOP` is used to keep track of the top element in the stack.
2. When initializing the stack, we set its value to -1 so that we can check if the stack is empty by comparing `TOP == -1`.
3. On pushing an element, we increase the value of `TOP` and place the new element in the position pointed to by `TOP`.
4. On popping an element, we return the element pointed to by `TOP` and reduce its value.
5. Before pushing, we check if the stack is already full
6. Before popping, we check if the stack is already empty