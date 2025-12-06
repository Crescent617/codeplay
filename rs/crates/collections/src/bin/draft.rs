fn main() {
    let x = String::from("Hello, world!");
    print(x);

    print(x);
}

fn print(s: String) {
    println!("{}", s);
}
