use std::mem;

fn str_array() {
    let mut languages: [&str; 3] = ["C++", "Rust", "Python"];
    languages[2] = "Golang";
    println!("{}", format!("Array size is {}. languages: {:?}", languages.len(), languages));  
}

fn int_array() {
    let mut height_list: [i32; 5] = [178, 175, 183, 180, 174];
    //Returns the size of the pointed-to value in bytes.
    let byte_size: usize = mem::size_of_val(&height_list);
    match byte_size == 4 * height_list.len() {
        true =>  println!("Array occupies {} bytes", byte_size),
        false => println!("Failed to get exact byte size."),
    }

    let mut first_element: &i32 =  height_list.get(0).unwrap();
    println!("first element: {}", first_element);
    first_element = &182;
    height_list[0] = *first_element;
    println!("height_list: {:?}", height_list);
}

pub fn array() {
    str_array();
    int_array();
}