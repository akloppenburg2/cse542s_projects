fn main() {

    #[cfg(oldexercise)]
    {
        // Declare int
        let firstint = 19;

        // Declare first ref
        let firstref = &firstint;

        // Declare second ref
        let secondref = &firstref;

        // Print vars
        println!("int: {}, reference 1: {}, reference 2: {}", firstint, firstref, secondref);

        // Compare refs
        let comparerefs = *firstref == **secondref;

        // Print comparison
        println!("result of comparison: {}", comparerefs);
    }

    #[cfg(oldexercise)]
    {
        // Declare int
        let mut firstint = 19;

        // Declare first ref
        let mut firstref = &firstint;

        // Print vars
        println!("int: {}, reference 1: {}", firstint, firstref);

        firstref = &27;

        // Print vars again
        println!("int: {}, reference 1: {}", firstint, firstref);
    }

    // Initialize string
    let mut firststring = "foo".to_string();

    {
        // Declare string
        let stringref = &mut firststring;

        // Print string
        println!("reference: {}", stringref);

        // Reassign string
        *stringref = "bar".to_string();

        // Print again
        println!("reference: {}", stringref);
    }

    // Print string
    println!("string: {}", firststring);

}
