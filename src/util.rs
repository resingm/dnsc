
/// Prints to STDOUT.
/// 
/// Note: Should be used only for the data generated. For anything user specific
/// use the `err(&str)` function.
/// 
pub fn log(msg: &str) {
    println!("{}", msg);
}


/// Prints to STDERR.
/// 
/// Should be used for any user-facing output. The data should be logged to STDOUT.
/// 
pub fn err(msg: &str) {
    eprintln!("{}", msg);
}
