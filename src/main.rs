#[cfg(not(it_works))]
compile_error!("rustflags not passed to rustc");

fn main() {
    println!("Hello, world!");
}
