fn main() {
    #[cfg(oldexercise)]
    {
        let var1 = ("strings", 5_u8);
        match var1 {
            (name, number) => {
                println!("name: {}, number: {}", name, number);
            }
        }
    }
    
    use std::str::FromStr;

    let literal = "list";

    #[cfg(oldexercise)]
    {
        match u8::from_str(&literal) {
            Ok(val) => {
                println!("valid 8 bit unsigned int {}", val);
            }
            Err(e) => {
                println!("not a valid 8 bit unsigned int! {}", e);
            }
        }
    }
    
    if let Ok(val) = u8::from_str(&literal) {
        println!("valid 8 bit unsigned int {}", val);
    } else {
        println!("not a valid 8 bit unsigned int!");
    }
}
