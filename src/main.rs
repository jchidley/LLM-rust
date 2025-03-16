//! Hello

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn main() {
    #[expect(unused_variables)] // `unused_variables` lint is not emitted
    let used = "I'm useful"; // the expectation is therefore unfulfilled
    println!("The `used` value is equal to: {used}");
    println!("Hello, world!");
}
