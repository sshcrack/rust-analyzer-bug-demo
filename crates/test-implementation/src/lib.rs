use test_trait::TestTrait;

pub struct TestStruct(pub usize);

impl TestTrait for TestStruct {
    fn test_function(&self) -> String {
        format!("TestStruct contains: {}", self.0)
    }
}
