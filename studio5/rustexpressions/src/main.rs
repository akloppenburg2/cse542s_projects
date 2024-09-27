fn main() {

    #[cfg(oldexercise)]
    {
        // Declare variable using arithmetic operators
        let math = 3 + 7 * 4 + 6;

        // Print result
        println!("value: {}", math);
    }

    #[cfg(oldexercise)]
    {
        // Initialize ints
        let firstnum = 7;
        let secondnum = 5;

        // Compare and print
        if firstnum < secondnum
        {
            println!("{}", secondnum);
        }
        else
        {
            println!("{}", firstnum);
        }
    }

    #[cfg(oldexercise)]
    {
        // Initialize tuple
        let numbers = (7, 5);

        // Match and initialize ints
        let firstnum = match numbers { (x, _) => x };
        let secondnum = match numbers { (_, y) => y };

        // Compare and print
        if firstnum < secondnum
        {
            println!("{}", secondnum);
        }
        else
        {
            println!("{}", firstnum);
        }
    }

    // Declare vector
    let mut numvec = Vec::new();

    // Push numbers
    for i in 0..10
    {
        numvec.push(i);
    }

    // Print vector
    println!("{:?}", numvec);

    // Iterate and print
    for num in &numvec
    {
        println!("{}", num);
    }

    // Print again
    println!("{:?}", numvec);
}
