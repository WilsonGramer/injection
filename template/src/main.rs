use template::MAGIC_STRING;

fn main() {
    let injected_data = unsafe { injected_data() };

    let string = std::str::from_utf8(injected_data).expect("Invalid string");

    println!("{}", string);
}

unsafe fn injected_data() -> &'static [u8] {
    // The magic string should have been replaced with two 'usize's -- the first
    // is the size of the injected data, and the second is a pointer to the data
    let magic_string_ptr = MAGIC_STRING.as_ptr() as *const usize;

    let data_size = *magic_string_ptr;
    let distance_from_magic_string_to_data = *magic_string_ptr.offset(1);
    let data_ptr = ((magic_string_ptr as usize) + distance_from_magic_string_to_data) as *const u8;

    // Read the injected data from the binary
    &*std::ptr::slice_from_raw_parts(data_ptr, data_size)
}
