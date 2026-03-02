fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            Some(10)
        }
        _ => {
            never_return_fn()
        }
    }
}

fn never_return_fn() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}