use std::thread;

fn main(){
    let mut handles = Vec::new();

    for x in 0..10{
        handles.push(
            //クロージャはデフォルトで借用（参照渡し）　、ムーブセマンティックスでもコピーセマンティックスでもない
            thread::spawn(move || {  
                println!("hello, world:{}",x);
            })
            //引数はムーブセマンティックス？
            /*
            thread::spawn(|t| {  
                println!("hello, world:{}",t);
            })
            */
        );
    }

    for handle in handles{
        let _ = handle.join();
    }
}