use std::ops::Range;

pub fn infinite_loop() {
    let mut counter: i32 = 0;
    loop {
        counter += 1;

        if counter >= 4 {
            break;
        }

        println!("Looping... {}", counter);
    }
}

pub fn while_loop() {
    let mut counter = 0;

    while counter <= 10 {
        counter += 1;

        println!("While loop {}", counter);
    }
}

pub fn for_loops() {
    // normal for loop
    for i in 1..5 {
        println!("For loop {}", i);
    }

    //let range: Range<i32> = 0..10;
    let range = 0..5;
    for i in range {
        println!("For loop with range {}", i);
    }

    // for loop with vectors
    let my_vector = vec!["Maça", "Laranja", "Pero"];

    // without the .iter(), we cannot access my_vector later
    for piece in my_vector.iter() {
        println!("Vector for loop: {}", piece);
    }

    println!("Accessing Laranja vector pos: {}", my_vector[1]);

    // for loop with enumerate
    for (index, value) in my_vector.iter().enumerate() {
        println!("Foor loop vector index is {} and value is {}", index, value);
    }
}