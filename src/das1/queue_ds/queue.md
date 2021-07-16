# Queue Data Structure

Queue follows the **First In First Out (FIFO)** rule - the item that goes in first is the item that comes out first.

## Basic Operations of Queue

A queue is an object (an abstract data structure - ADT) that allows the following operations:
- **Enqueue**: Add an element to the end of the queue.
- **Dequeue**: Remove an element from the front of the queue.
- **IsEmpty**: Check if the queue is empty.
- **IsFull**: Check if the queue is full.
- **Peek**: Get the value of the front of the queue without removing it.

## Working of Queue
Queue operations work as follows:

- Two pointers `FRONT` and `REAR`.
- `FRONT` tracks the first element of the queue.
- `REAR` tracks the last element of the queue.
- initially, set the value of `FRONT` and `REAR` to -1.

### Enqueue Operation
- check if the queue is full
- for the first element, set the value of `FRONT` to 0
- increase the REAR index by 1
- add the new element in the position pointed to by `REAR`

### Dequeue Operation
- check if the queue is empty
- return the value pointed by `FRONT`
- increase the `FRONT` index by 1
- for the last element, reset the values of `FRONT` and `REAR` to -1