
struct MyStruct {

}
impl Self {
    // This is just being used to test the parser. In the real world, this wouldn't compile.
    met my_method();
    met my_method() -> i32;
    met my_method() {}
    met my_method() -> i32 {}
}
