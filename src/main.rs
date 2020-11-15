mod loops;
mod enums;

fn main() {
    loops::infinite_loop();
    loops::while_loop();
    loops::for_loops();

    enums::var_enum();

    println!("Hello, world!");
}
