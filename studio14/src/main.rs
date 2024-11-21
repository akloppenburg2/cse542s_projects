fn main() {
    // Declare string
    let mut num_string: String;

    // Declare and initialize vector
    let mut primes: Vec<bool> = (0..=99).map(|x| (x % 2 == 1 && x > 2) || x == 2).collect(); 

    // Construct table
    for num in 0..=99
    {
        // If we hit a value that's true, mark all of its multiples false
        if primes[num]
        {
            ((num * num)..=99).step_by(2 * num).for_each(|x| primes[x] = false)
        }

        // If we hit a multiple of 10, insert a newline, otherwise concatenate normally
        // If the value is false in the vector, simple print spaces
        if primes[num]
        {
            if num % 10 == 9
            {
                num_string = format!("{:>2}\n", num);
            }
            else
            {
                num_string = format!("{:>2} ", num);
            }
        }
        else
        {
            if num % 10 == 9
            {
                num_string = format!("  \n");
            }
            else
            {
                num_string = format!("   ");
            }
        }

        // Print value
        print!("{}", num_string);
    }
}
