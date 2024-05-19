use derive_more::{Add, Display, From, Into};

#[derive(Debug, PartialEq, From, Add)]
struct MyInt(i32);

#[derive(PartialEq, From, Into)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, From, Add, Display)]
enum MyEnum {
    #[display(fmt = "int: {}", _0)]
    Int(i32),
    Uint(u32),
    #[display(fmt = "---nothing---")]
    Nothing,
}

fn main() {
    let my_int: MyInt = i32::into(68);
    println!("myint: {:?}", my_int);

    let v1 = MyEnum::Int(2);
    println!("v1: {}", v1);

    let nt = MyEnum::Nothing;
    println!("nt: {}", nt);

    let v2 = MyEnum::Int(2);
    println!("v1 + v2: {:?}", v1 + v2);
}
