// pub struct String {
//     vec: Vec<u8>,
// }


fn simple_string() {
    //define sentence with mut to be able to push str to it
    let mut sentence: String = String::from("Rust is a programming language");
    sentence.push_str(" which is mainly used in system programming.");
    println!("sentence: {}", sentence);

    let count: usize = sentence.matches("programming").count();
    println!("programming appears {} times", count);
}

fn substrings() {
    let task: String = String::from("Learn the String type of Rust.");
    let length:usize = task.len();
    let language: &str = &task[length-5..length-1];
    println!("sliced part is: {}", language);
}

fn string_with_loops() {
    let code: String = String::from("kkRkuskt kis qukkitek fkun.");
    let code_str: &str = code.as_str();
    let mut characters: Vec<&str> = code_str.split("").collect();
    let mut result: String = String::from("");
    for character  in characters.iter_mut() {
        if character != &mut "k" {
            result.push_str(character);
        }
    }
    println!("my secret message is: {}", result);
}

fn combine_strings() {
    let language: String = String::from("Rust");
    let additional: &str = ", Golang, Python";
    let languages: String = language + additional;
    let count: i32 = 3;
    let sentence: String = format!("My favorite languages are these {} languages: {}", count, languages);
    println!("sentence: {}", sentence);
}

// fn string_with_vec() {
//     let countries: Vec<String> = vec![
//         String::from("United states"),
//         String::from("Germany"),
//         String::from("Russia"),
//     ];
//     println!("countries: {:?}", countries);

//     //Use iter()
//     for country in countries.iter() {
//         match country.as_str() {
//             "United states" => println!("It's United states"),
//             "Germany" => println!("It's Germany"),
//             "Russia" => println!("It's Russia"),
//             _ => println!(""),
//         }
//     }
//     println!("countries: {:?}", countries);

//     //use into_iter()
//     for country in countries.into_iter() {
//         match country.as_str() {
//             "United states" => println!("It's United states"),
//             "Germany" => println!("It's Germany"),
//             "Russia" => println!("It's Russia"),
//             _ => println!(""),
//         }
//     }
//     //println!("countries: {:?}", countries);
//     println!("can not print countries because into_iter() consumes each value of Vec<String>.")
// }

pub fn string() {
    simple_string();
    substrings();
    string_with_loops();
    combine_strings();
    //string_with_vec();
}