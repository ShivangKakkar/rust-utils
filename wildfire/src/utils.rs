// Get and Print Type of Variable
pub fn type_of<T>(_: &T) -> &'static str {
    println!("{}", std::any::type_name::<T>());
    return std::any::type_name::<T>();
}

// Print Generic
pub fn print<T: std::fmt::Debug>(x: &T) {
    println!("{:?}", x);
}

// Pretty-Print Generic
pub fn pprint<T: std::fmt::Debug>(x: &T) {
    println!("{:#?}", x);
}
