use test_implementation::TestStruct;

fn main() {
    let test = TestStruct(42);
    // The test_function should be suggested by rust-analyzer but is not.
    test.test_function();

    println!("Hello, world!");
}
