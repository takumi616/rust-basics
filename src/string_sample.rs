fn simple_string() {
    let mut greeting: String = String::from("Hello");
    println!("greeting: {}", greeting);

    greeting.push_str(", world");
    println!("greeting: {}", greeting);
}

fn string_with_vec() {
    let countries: Vec<String> = vec![
        String::from("United states"),
        String::from("Germany"),
        String::from("Russia"),
    ];
    println!("countries: {:?}", countries);

    //Use iter()
    for country in countries.iter() {
        match country.as_str() {
            "United states" => println!("It's United states"),
            "Germany" => println!("It's Germany"),
            "Russia" => println!("It's Russia"),
            _ => println!(""),
        }
    }
    println!("countries: {:?}", countries);

    //use into_iter()
    for country in countries.into_iter() {
        match country.as_str() {
            "United states" => println!("It's United states"),
            "Germany" => println!("It's Germany"),
            "Russia" => println!("It's Russia"),
            _ => println!(""),
        }
    }
    //println!("countries: {:?}", countries);
    println!("can not print countries because into_iter() consumes each value of Vec<String>.")
}

pub fn string() {
    simple_string();
    string_with_vec();
}