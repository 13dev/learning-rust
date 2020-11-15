pub fn define_tuples() {
    let my_tuple = (1, 20, "My String", 40);

    println!("Accessing tuple... ({})", my_tuple.1);

    //Mass assignment

    let (a, b, c) = ((1, 2, 3), "1", 1);

    println!("Accessing tuple 2... ({})", a.1);
    println!("Accessing tuple 3... ({})", b);

}