use std::collections::HashMap;

pub fn hashmap() { 
    let mut languages = HashMap::new();

    languages.insert("Rust", "system programming");
    languages.insert("Go", "web backend");
    languages.insert("Python", "data science");

    println!("Current hash map: {:?}", languages);

    // Takes a reference and returns Option<&V>
    match languages.get(&"Rust") {
        Some(&usedin) => println!("Rust is used in {}", usedin),
        None => println!("Rust is not in hash map."),
    }

    // If the map did not have this key present, None is returned.
    // If the map did have this key present, the value is updated, and the old value is returned.
    match languages.insert("Rust", "blockchain") {
        Some(usedin) => println!("Update the value and this is old one: {}", usedin),
        None => println!("Failed to insert.")
    }

    //Remove value from hash map
    languages.remove(&"Go"); 

    println!("Current hash map: {:?}", languages);

    //Gets key value pairs using for loops
    for (language, &usedin) in languages.iter() {
        println!("{} is mainly used in {}.", language, usedin); 
    }
}