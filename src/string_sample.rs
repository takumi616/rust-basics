pub fn string() {
    let mut greeting: String = String::from("Hello");
    println!("greeting: {}", greeting);

    greeting.push_str(", world");
    println!("greeting: {}", greeting);
}