use tomlstruct::tomlstruct;

tomlstruct!{
    [Hello]
    name = "hello"
    version = 1.0
}

fn main() {
    //println!("yahhoooooo");
    //dbg!("iyaaaaa");
    let _ = Hello {
        name: String::from("hello"),
        version: 1.0,
    };
}
