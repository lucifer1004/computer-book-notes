fn f(x: bool) -> bool {
    !f(x)
}

fn main() {
    println!("{}", f(true));
}
