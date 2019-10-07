use std::io::stdin;

mod alpha;
mod sample;

fn main() {
    loop {
        println!("\nPlease enter your name.!\n");

        // User Type name Variable
        let mut input_name = String::new();

        // User Input Method here
        stdin()
            .read_line(&mut input_name)
            .expect("Failed to read line");

        let split_name: Vec<&str> = input_name.trim().split("").collect();

        for i in split_name {
            if i == "a" {
                alpha::a();
            } else if i == "b" {
                alpha::b();
            } else if i == "c" {
                alpha::c();
            } else if i == "d" {
                alpha::d();
            } else if i == "e" {
                alpha::e();
            } else if i == "f" {
                alpha::f();
            } else if i == "g" {
                alpha::g();
            } else if i == "h" {
                alpha::h();
            } else if i == "i" {
                alpha::i();
            } else if i == "j" {
                alpha::j();
            } else if i == "k" {
                alpha::k();
            } else if i == "l" {
                alpha::l();
            } else if i == "m" {
                alpha::m();
            } else if i == "n" {
                alpha::n();
            } else if i == "o" {
                alpha::o();
            } else if i == "p" {
                alpha::p();
            } else if i == "q" {
                alpha::q();
            } else if i == "r" {
                alpha::r();
            } else if i == "s" {
                alpha::s();
            } else if i == "t" {
                alpha::t();
            } else if i == "u" {
                alpha::u();
            } else if i == "v" {
                alpha::v();
            } else if i == "w" {
                alpha::w();
            } else if i == "x" {
                alpha::x();
            } else if i == "y" {
                alpha::y();
            } else if i == "z" {
                alpha::z();
            }
        }
    }
}
