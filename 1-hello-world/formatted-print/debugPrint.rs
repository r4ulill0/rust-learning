
#[derive(Debug)]
struct DebugStruct(i32);

#[derive(Debug)]
struct NestedStruct(DebugStruct);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("{:?} debug print of a number", 3);
    println!("{1:?} {0:?} debug print of strings", "strings", "some");

    println!("DebugStruct debug print:\n{:?}", DebugStruct(3));
    println!("NestedStruct debug print:\n{:?}", NestedStruct(DebugStruct(3)));

    // Now some "pretty" printing
    let name = "Pretty Peter";
    let age = 23;
    let peter = Person{name, age};
    println!("Lets see some pretty printing on a custom made struct: \n");
    println!("{:#?}", peter);
}
