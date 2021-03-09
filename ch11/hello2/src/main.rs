use tomlstruct::tomlstruct;

tomlstruct! {
    [Hello]
    name = "Hello"
    version = 1.0
}
fn main() {
    let x= Hello {
        name: String::from("hello"),
        version: 1.0,
    };
    println!("{},{}", x.name, x.version);
    
}
