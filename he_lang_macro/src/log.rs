use std::fmt::Display;




pub fn handle_error<T, E: Display>(e: std::result::Result<T, E>){
    if let Err(e) = e{
        println!("{e}")
    } else {
        panic!("handle_error is handling an error which is not an error")
    }
}