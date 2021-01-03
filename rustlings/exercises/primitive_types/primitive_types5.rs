// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!


fn main() {
    let poomcpeter = ("peter", 2.5);
    let (name,age) = poomcpeter;

    println!("{} is {} years old.", name, age);
}
