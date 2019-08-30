
struct MyStruct {

}
impl Self {
    // This is just being used to test the parcer. In the real world, this wouldn't compile.
    met my_method();
    met my_method() -> i32;
    met my_method() {}
    met my_method() -> i32 {}

    get my_getter:default -> i32;
    get my_getter(&self) -> i32 {}
    get my_getter(&self) -> i32;

    set my_setter:default;
    set my_setter(value: i32) {}
    set my_setter(value: i32);
}
