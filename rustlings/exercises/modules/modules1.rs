// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)


// use modules1::sausage_factory; put this in a different file to have access to sausage factory function

mod sausage_factory { // module: allows us to split stuff up from different files 
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
