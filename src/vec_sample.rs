fn vec_with_string() {
    let mut genais: Vec<String> = Vec::new();
    genais.push(String::from("Llama"));
    genais.push(String::from("Gemini"));
    println!("genais: {:?}", genais);
    
    let mut additional: Vec<String> = Vec::new();
    additional.push(String::from("Chatgpt"));
    additional.push(String::from("Cohere"));
    println!("additional: {:?}", additional);

    genais.append(&mut additional);
    println!("genais: {:?}", genais);
}

fn vec_with_str() {
    let mut languages: Vec<&str> = vec!["Rust", "Golang", "Python"];
    println!("languages: {:?}", languages);

    languages.push("C++");
    println!("languages: {:?}", languages);
}

pub fn vec() {
    vec_with_string();
    vec_with_str();
}