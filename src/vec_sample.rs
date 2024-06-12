//Access the elemen of Vector
fn access_elements() {
    let languages: Vec<&str> = vec!["C++", "Rust", "Python"];
    let language: &str = languages[1];
    println!("accessed element: {}", language);

    //If given a position, returns a reference to the element 
    //at that position or None if out of bounds
    let another_language:Option<&&str> = languages.get(0);
    match another_language {
        Some(accessed) => println!("accessed element using get method: {}", accessed),
        None => println!("not found"),
    }

    //If given a range, returns the subslice corresponding to that range, 
    //or None if out of bounds.
    let languages_slice:Option<&[&str]> = languages.get(1..3);
    match languages_slice {
        Some(accessed_languages) => println!("accessed elements creating slice: {:?}", accessed_languages),
        None => println!("not found"),
    }
}

//Modify vector
fn modify_vectors() {
    let mut data_structures: Vec<&str> = vec!["stack", "queue", "linkedlist"];
    println!("data_structures: {:?}", data_structures);

    data_structures.push("hashtable");
    println!("data_structures: {:?}", data_structures);

    data_structures[1] = "array";
    println!("data_structures: {:?}", data_structures);

    let popped: Option<&str> = data_structures.pop();
    match popped {
        Some(popped_data_structure) => println!("Popped data structure: {}", popped_data_structure),
        None => println!("not found"),
    }
    println!("data_structures: {:?}", data_structures);

    let removed: &str = data_structures.remove(1);
    println!("removed data structure: {}", removed);
    println!("data_structures: {:?}", data_structures);
}

pub fn vec() {
    access_elements();
    modify_vectors();
}