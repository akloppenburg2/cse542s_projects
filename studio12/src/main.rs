use std::fmt::Debug;

#[derive(Debug, Default)] 
struct MyStruct<T> where T: Debug + Clone
{
    one: T,
    two: T,
}

impl<T> MyStruct<T> where T: Debug + Clone
{
    fn new(one: &T, two: &T) -> MyStruct<T>
    {
        MyStruct{one: one.clone(), two: two.clone()}
    }
}

//#[cfg(oldexercise)]
impl<T> Drop for MyStruct<T> where T: Debug + Clone
{
    fn drop(&mut self)
    {
        println!("{:?} is being dropped!", self)
    }
}



fn main()
{
    let strings  = MyStruct::new(&"first".to_string(), &"second".to_string());
    let ints = MyStruct::new(&(15 as u16), &(30 as u16));
    println!("{:?}, {:?}", strings, ints);

    let default_strings  = MyStruct::new(&Default::default(), &"another string".to_string());
    let default_ints = MyStruct::new(&Default::default(), &(79 as u16));
    println!("{:?}, {:?}", default_strings, default_ints);
}
