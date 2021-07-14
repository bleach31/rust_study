struct A(bool);

impl Drop for A {
    fn drop(&mut self){
        if self.0 {
            eprintln!("aaaaa...");
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut flag = A(true);

    assert!(false); //ここでパニックが起きて、Aのdrop処理が走って、プログラムも終了

    flag.0 = false;
    eprintln!("Success!");
}
