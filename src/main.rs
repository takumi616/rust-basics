mod array_slice_sample;
mod string_sample;
mod hashmap_sample;
mod for_sample;


fn main() {
    println!("*** Array and Slice ***");
    array_slice_sample::array();
    println!("*************************");
    println!("");

    println!("*** For loops ***");
    for_sample::iter();
    for_sample::into_iter();
    for_sample::iter_mut();
    println!("************************");
    println!("");

    println!("*** Hash map ***");
    hashmap_sample::hashmap();
    println!("************************");
    println!("");

    println!("*** Char, str and String ***");
    string_sample::string();
    println!("************************");
    println!("");
}
