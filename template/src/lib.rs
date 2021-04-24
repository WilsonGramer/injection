/// This string contains a UUID that will almost surely be present only once in
/// the binary, so we can search for the address of this string and replace it
/// with the data we need. Whatever its value, the magic string must be at least
/// `2 * mem::size_of<usize>()` bytes in length; that is, it should be able to
/// contain a `usize` representing the size of the injected data and a `usize`
/// representing the distance from the magic string to the injected data in the
/// binary.
#[used]
pub static MAGIC_STRING: &[u8] = b"D64921A4-EF6D-45E5-9964-D4C5CF48E4C2";
