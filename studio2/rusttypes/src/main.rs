fn main()
{
    // Declare vars
    let mut bigint: u8 = 1;         // From step 2
    let mut counter = 0;            // From step 4

    // Declare and push into vector
    let mut mult_vec = Vec::new();  // From step 7
    mult_vec.push(bigint);

    // Code from step 2
    #[cfg(oldexercise)]
    {
        loop
        {
            bigint *= 2;
            println!("{}", bigint);
        }
    }

    // Code from step 3
    #[cfg(oldexercise)]
    {
        while bigint.checked_mul(2) != None
        {
            bigint *= 2;
            println!("{}", bigint);
        }
    }

    // Code from step 4
    #[cfg(oldexercise)]
    {
        // The .1 here corresponds to the boolean value returned by overflowing_mul indicating if it overflowed
        while bigint.overflowing_mul(2).1 != true
        {
            bigint *= 2;
            counter += 1;
            println!("{}, number of doublings is: {}", bigint, counter);
        }
    }

    // Code from step 5
    #[cfg(oldexercise)]
    {
        for _ in 0..8
        {
            bigint = bigint.saturating_mul(2);
            counter += 1;
            println!("{}, number of doublings is: {}", bigint, counter);
        }
    }

    // Code from step 6
    #[cfg(oldexercise)]
    {
        for _ in 0..8
        {
            bigint = bigint.wrapping_mul(2);
            counter += 1;
            println!("{}, number of doublings is: {}", bigint, counter);
        }
    }

    for _ in 0..8
    {
        bigint = bigint.wrapping_mul(2);
        counter += 1;
        println!("value: {}, number of doublings is: {}", bigint, counter);
        mult_vec.push(bigint);
    }

    // Sort and print vector
    mult_vec.sort();
    println!("sorted vector: {:?}", mult_vec)
}
