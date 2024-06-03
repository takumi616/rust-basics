mod array_slice_sample;
mod for_sample;


fn main() {
    array_slice_sample::array();

    for_sample::iter();
    for_sample::into_iter();
    for_sample::iter_mut();
    println!("Hello, world!");
}
