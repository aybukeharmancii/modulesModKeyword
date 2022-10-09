mod example {
    fn chicken() {
        println!("Chicken!");
    }

    pub fn print_message() {
        chicken();
        println!("How's it going?");
    }

    pub mod water {
        pub fn print_message() {
            println!("Water!");
        }
    }
}

fn main() {
    example::print_message();
    example::water::print_message();
}
