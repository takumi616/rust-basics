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

fn edit_slice() {
    let mut nums: Vec<i32> = vec![60, 66, 160, 166, 260];
    println!("nums: {:?}", nums);

    let nums_slice: &mut [i32] = &mut nums[0..4]; 
    nums_slice[0] = 16;
    println!("nums: {:?}", nums);

    let _removed: i32 = nums.remove(2);
    println!("nums: {:?}", nums);

    nums.insert(4, 266);
    println!("nums: {:?}", nums);
}

pub fn slice() {
    slice_from_array();
    slice_from_vec();
    edit_slice();
}