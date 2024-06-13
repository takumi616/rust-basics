use std::mem;

pub fn array() {
    // Fixed-size array 
    //let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let num_list: [i32; 4] = [6, 16, 26, 36];
    println!("First element of the num_list: {}", num_list[0]);
    println!("Second element of the num_list: {}", num_list[1]);
    println!("Third element of the num_list: {}", num_list[2]);

    // `len()` returns the count of elements in the array.
    println!("Number of elements in array: {}", num_list.len());

    // Arrays are stack allocated. 
    //Expected 16bytes
    println!("Array occupies {} bytes", mem::size_of_val(&num_list));

    // All elements can be initialized to the same value.
    let same_value: [i32; 5] = [6; 5];
    println!("Same value is repeated: {:?}", same_value);
}