trait MyTrait {
    met my_method();
    met my_method() -> i32;
    met my_method() {}
    met my_method() -> i32 {}

    get my_getter:default;
    get my_getter(&self) -> i32 {}
    get my_getter(&self) -> i32;

    set my_setter:default;
    set my_setter(value: i32) {}
    set my_setter(value: i32);
}

