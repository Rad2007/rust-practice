fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit())
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}