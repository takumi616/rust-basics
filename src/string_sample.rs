pub fn string() {
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