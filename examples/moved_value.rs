fn bar(s: String) {
}

fn main() {
    let foo = "hello world".to_owned();
    bar(foo);
    bar(foo);
}
