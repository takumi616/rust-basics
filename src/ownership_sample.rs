// ****** Stack ******************************************************
// The stack stores values in the order it gets them 
// and removes the values in the opposite order. 
// This is referred to as last in, first out.
// All data stored on the stack must have a known, fixed size
// *******************************************************************


// ****** Heap ***************************************************************************
// When you put data on the heap, you request a certain amount of space. 
// The memory allocator finds an empty spot in the heap that is big enough, 
// marks it as being in use, and returns a pointer, which is the address of that location. 
// Because the pointer to the heap is a known, fixed size, 
// you can store the pointer on the stack, but when you want the actual data, 
// you must follow the pointer. 
// ***************************************************************************************



// Pushing to the stack is faster than allocating on the heap 
// because the allocator never has to search for a place to store new data



// *** Ownership rules *****************************************
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
// *************************************************************
