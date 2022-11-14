#[ctor::ctor]
fn ctor() {
    println!("Hello from dll");
}