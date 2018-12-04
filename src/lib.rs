pub mod Test {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn bad_add(a: i32, b: i32) -> i32 {
        a - b
    }
}