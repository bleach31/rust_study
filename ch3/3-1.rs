fn main() {
    println!("Hello, world!");
    let s1: String = String::from("hello, world");
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    let mut t = (1, "2");
    
    t.0 = 2;
    t.1 = "3";
    
    let mut a: [i32; 3] = [0,1,2];
    let b: [i32; 3] = [0; 3]; // all zero
    
    a[1] = b[1];
    a[2] = b[2];
    
    println!("{:?}", &a[0..2]); // start to end - 1 
    // https://doc.rust-jp.rs/rust-by-example-ja/hello/print.html
    //fmt::Debug: は、{:?}というマーカーを使用し、デバッギング目的に使われます。
    //fmt::Display: は {}というマーカーを使用し、より美しく、ユーザフレンドリーに表示します。
    //:?はデバッグ用に全部出しまっせって感じかな
    
    // !マークは関数じゃなくてマクロ呼び出し　by オライリー本
    
    struct Person{
        name: String,
        age: u32,
    };
    
    let p = Person{
        name: String::from("john"),
        age: 8
    };
    
    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown  {x: i32, y: i32},
    };
    
    let e1 = Event::Quit;
    let e2 = Event::MouseDown{ x: 10, y:10};
    //値付き列挙体まじ便利
    
    let result: Result<i32, String> = Ok(200);
    
    match result{
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    };
    
    //　逐一Resultを定義しないと怒られる value used here after partial move
    // IFの書き方説明せえや・・・
    let result: Result<i32, String> = Ok(200);
    if let Ok(code) = result {
        println!("code: {}", code);
    }
    
    let result: Result<i32, String> = Ok(200);
    println!("code: {}", result.unwrap_or(-1));
    
    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1));
    
    
    fn func(code: i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100)
    }
    
    let result: Result<i32, String> = Ok(200);
    let next_result: Result<i32, String> = result.and_then(func);

    let result: Result<i32, String> = Err("error".to_string());
    let next_result: Result<i32, String> = result.and_then(func);
    
    
    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let code = result?;
        println!("code: {}", code);
        Ok(100)
    }
    //特に例題はない・・・

    let v1 = vec![0; 5];
    let v1 = vec![1,2,3,4,5];    
    println!("{}", v1[0]);

    let v1 = vec![1,2,3,4,5];    
    for element in &v1{
        println!("{}", element);
        
    }

    let byte_array = [b'h',b'e',b'l',b'l',b'o'];// bは多分アスキーコードにするやつ

    //エラー　doesn't have a size known at compile-time
    /*
    print(byte_array);
    fn print(s: [u8]){
        println!("{:?}",s);
    }
    */

    print(Box::new(byte_array));
    fn print(s: Box<[u8]>){
        println!("{:?}",s);
    }

    

}
    
    
    
    
    
    
    
    
    
    