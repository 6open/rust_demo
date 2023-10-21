mod first_module {
    pub fn print_chars_a_to_Z() {
        println!("print_chars_a_to_Z\n");
        for ch in ('Z'..='a').rev() {
            println!("{}", ch);
        }
    }
}

mod second_module {
    pub fn print_chars_A_to_z() {
        println!("print_chars_A_to_z\n");
        for ch in 'A'..='Z' {
            println!("{}", ch);
        }
    }
}

fn main(){
    first_module::print_chars_a_to_Z();
    second_module::print_chars_A_to_z();
}