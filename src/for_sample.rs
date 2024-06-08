//iter - This borrows each element of the collection through each iteration. 
//Thus leaving the collection untouched and available for reuse after the loop.
pub fn iter() {
    let genai_list = vec!["llama", "gemini", "chatgpt"];

    for genai in genai_list.iter() {
        match genai {
            &"llama" => println!("Llama is developed by Meta."),
            &"gemini" => println!("Gemini is developed by Google."),
            &"chatgpt" => println!("Chatgpt is developed by OpenAI."),
            _ => println!("Unexpected genai name."),
        }
    }
    
    //Can output this line because iter method just borrows each element 
    //of genai_list and does not change it.
    println!("Gen AI List: {:?}", genai_list);
}

//into_iter - This consumes the collection so that 
//on each iteration the exact data is provided.
pub fn into_iter() {
    let languages = vec!["Rust", "Golang", "Python"];

    for language in languages.into_iter() {
        match language {
            "Rust" => println!("Rust is mainly used for system programming."),
            "Golang" => println!("Golang is mainly used for web backend."),
            "Python" => println!("Python is mainly used for data science."),
            _ => println!("Unexpected programming language name."),
        }
    }
    
    //Once the collection has been consumed it is no longer available 
    //for reuse as it has been 'moved' within the loop.

    // Failed to output following line.
    //println!("languages: {:?}", languages);
    println!("In this case, can not output the collection.")
}

//iter_mut - This mutably borrows each element of the collection, 
//allowing for the collection to be modified in place.
pub fn iter_mut() {
    let mut colors = vec!["Red", "Green", "White"];
    println!("Before modifying: {:?}", colors);

    for color in colors.iter_mut() {
        *color = match color {
            &mut "Red" => "Blue",
            &mut "Green" => "Black",
            &mut "White" => "Purple",
            _ => "Orange",
        }
    }

    println!("After modifying: {:?}", colors);
}
