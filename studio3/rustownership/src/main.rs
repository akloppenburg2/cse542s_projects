fn main() {

    // Code from step 2
    #[cfg(oldexercise)]
    {
        // Declare first two ints
        let first: u8 = 9;
        let second: u8 = 13;

        // Assign third int
        let third: u8 = second;

        // Print all three ints
        println!("the three ints are: {}, {}, and {}", first, second, third);
    }

    // Declare first two strings
    let first: String = "foo".to_string();
    let mut second: String = "bar".to_string();

    // Assign third string
    let mut third: String = second;
    second = "baz".to_string();

    // Print all three strings
    println!("the three strings are: {}, {}, and {}", first, second, third);

    // Assign and print again
    second = third;
    third = "qux".to_string();
    println!("the three strings are: {}, {}, and {}", first, second, third);

    // Declare vector, push and print
    let mut stringvec = Vec::new();
    stringvec.push(first);
    stringvec.push(second);
    stringvec.push(third);
    println!("the string vector is: {:?}", stringvec);

    // Initialize and assign fourth string, then print both it and the vector
    let fourth: String = stringvec.pop().unwrap();
    println!("the fourth string is: {}", fourth);
    println!("the string vector is: {:?}", stringvec);

}
