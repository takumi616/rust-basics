fn slice_from_array() {
    let nums: [i32; 6] = [6, 16, 26, 36, 46, 56];
    let nums_slice: &[i32] = &nums[1..5]; 
    println!("nums_slice: {:?}", nums_slice); //output => [16, 26, 36, 46]
}

fn slice_from_vec() {
    let languages: Vec<&str> = vec!["Rust", "C++", "Golang", "Python", "Java", "Ruby"];
    let languages_slice: &[&str] = &languages[0..3];
    println!("languages_slice: {:?}", languages_slice); //output => ["Rust", "C++", "Golang"]
}

pub fn slice() {
    slice_from_array();
    slice_from_vec();
}