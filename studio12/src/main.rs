#[derive(Debug)]
struct StringStruct
{
    string: String,
}

impl StringStruct
{
    fn new(string: &String) -> StringStruct
    {
        StringStruct{string: string.to_string()}
    }
}

fn main()
{
    let first  = StringStruct::new(&"first".to_string());
    let second = StringStruct::new(&"second".to_string());
    println!("{:?}, {:?}", first, second);
}
