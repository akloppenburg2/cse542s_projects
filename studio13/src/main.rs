struct Functions<T>
{
    num: T,
    sine: fn(T) -> T,
    cosine: fn(T) -> T,
}

fn main() {
    let funcs = Functions{num: 0.0, sine: f64::sin, cosine: f64::cos};

    let funcs_closure = |funcs: Functions<f64>| (funcs.cosine)((funcs.sine)(funcs.num));

    println!("{}", funcs_closure(funcs));

    let mut hello: String = "Hello".to_string();

    let string_closure = |mut string: String| {string.push_str(", world!"); string};

    println!("{}", string_closure(hello));
}
