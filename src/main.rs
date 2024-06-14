mod array_sample;
mod for_sample;
mod hashmap_sample;
mod ownership_sample;
mod slice_sample;
mod string_sample;
mod vec_sample;


fn main() {
    println!("*** Array and Slice ***");
    array_sample::array();
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

    println!("*** Vec ***");
    vec_sample::vec();
    println!("************************");
    println!("");

    println!("*** String ***");
    string_sample::string();
    println!("************************");
    println!("");

    println!("*** Slice ***");
    slice_sample::slice();
    println!("************************");
    println!("");    
}
