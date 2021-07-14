use std::panic::catch_unwind;

// 最大値と最小値の差を出力する処理（バグが２つある）
fn print_range(x: &[i32]){
    let min = x.iter().min().unwrap();
    let max = x.iter().max().unwrap();
    eprintln!("max - min = {}",max - min);
}


fn main() {
    
    let requests = vec![
        vec![1,2,3],
        vec![],
        vec![2000000000, -2000000000],
        vec![0,42],
    ];

    for request in &requests{
        let result = catch_unwind(||{
            print_range(request);
        });
        if let Err(payload) = result {
            println!("{:?}",payload);
            //println!("print_range failed");
        }else{
            println!("success");
        }
    }
}
