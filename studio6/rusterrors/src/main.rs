fn main() {
    
    #[cfg(oldexercise)]
    {
        let expression = 2 + 2 * 10 - 22;
        let divide_by_zero = 10 / expression;
        println!("expression: {}", divide_by_zero);
    }
    
    let dem = 2 + 2 * 10 - 20;
    let num = 10;

    match expression(&num, &dem) {
        Ok(x) => {
            println!("value of division: {}", x);
        }
        Err(e) => {
            println!("Error: {} divide by zero occured but was handled without a panic", e);
        }
    }
}

fn expression(num: &i32, dem: &i32) -> Result<isize, isize> {
    if *dem == 0 {
        Err(-1)
    } else {
        Ok((num/dem) as isize)
    }
}