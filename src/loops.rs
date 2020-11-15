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