fn main() {
    #[cfg(oldexercise)]
    {
        // initialize tuple
        let var1 = ("strings", 5_u8);

        // decompose tuple with match statement
        match var1 {
            (name, number) => {
                println!("name: {}, number: {}", name, number);
            }
        }
    }
    
    use std::str::FromStr;

    // initialize string literal
    let literal = "list";

    #[cfg(oldexercise)]
    {
        // convert string to u8 and capture Result using match statement
        match u8::from_str(&literal) {
            // print if valid
            Ok(val) => {
                println!("valid 8 bit unsigned int {}", val);
            }
            // print error if not valid
            Err(e) => {
                println!("not a valid 8 bit unsigned int! {}", e);
            }
        }
    }
    
    // convert string to u8 and capture Result using if let statment
    if let Ok(val) = u8::from_str(&literal) {
        // print if valid
        println!("valid 8 bit unsigned int {}", val);
    } else {
        // print error if not valid
        println!("not a valid 8 bit unsigned int!");
    }
}
