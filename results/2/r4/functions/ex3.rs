fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    std::process::exit(0);
}

fn main() {
    never_return();
    println!("Failed!");
}

fn never_return() -> ! {
    loop {
        std::process::exit(0);
    }
}