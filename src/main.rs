use async_std::task;

fn f2() -> u8 {
    task::block_on(async { 42 })
}

fn f1() -> u8 {
    task::block_on(async { f2() })
}
fn main() {
    println!("Hello, world: {}", f1());
}
