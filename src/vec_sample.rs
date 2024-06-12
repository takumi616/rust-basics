// fn vec_with_string() {
//     let mut genais: Vec<String> = Vec::new();
//     genais.push(String::from("Llama"));
//     genais.push(String::from("Gemini"));
//     println!("genais: {:?}", genais);
    
//     let mut additional: Vec<String> = Vec::new();
//     additional.push(String::from("Chatgpt"));
//     additional.push(String::from("Cohere"));
//     println!("additional: {:?}", additional);

//     genais.append(&mut additional);
//     println!("genais: {:?}", genais);
// }

// fn vec_with_str() {
//     let mut languages: Vec<&str> = vec!["Rust", "Golang", "Python"];
//     println!("languages: {:?}", languages);

//     languages.push("C++");
//     println!("languages: {:?}", languages);
// }

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

pub fn vec() {
    access_elements();
}