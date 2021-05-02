use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const N: usize = 81;

// 盤面でルールを満たしていることを検査する
fn is_valid(result: [u8; N], p: usize, v: u8) -> bool {
    let y = p / 9;
    let x = p % 9;
    
    //行または列にvが含まれたらダメ
    for i in 0..9{
        if result[9 * i + x] == v ||  result[9 * y + i] == v {
            return false;
        } 
    }

    let block_y = y/3;
    let block_x = x/3;
    //3x3のところにvが含まれたらダメ
    for i in 0..3{
        for j in 0..3{
            if result[9 * ( 3 * block_y + i) + (3 * block_x + j) ] == v {
                 return false;
            }
        }
    }
    return true;
}

#[wasm_bindgen]
pub fn solve(problem: Vec<u8>) -> Vec<u8>{
    solve_inner(problem)
}

fn solve_inner(problem: Vec<u8>) -> Vec<u8>{
    let mut result = [0; N]; //81マス as 1次元配列
    
    let mut stack = vec![];
    for i in 0..N{
        if problem[i] > 0 {
            result[i] = problem[i];
        } else if stack.is_empty(){
            stack.push((false, i, 1));
        }
    }

    let mut is_failing = false;
    while !stack.is_empty(){
        let (is_back, p, v) = stack.pop().unwrap();
        // 戻りがけの処理
        if is_back && is_failing {
            result[p] = 0; //未確定に戻す
            if v < 9{
                stack.push((false, p, v + 1)); //次の候補
            }
            continue;
        }
        // 行きがけの処理
        if !is_valid(result, p, v){
            if v < 9 {
                stack.push((false, p, v + 1)); //次の候補
            } else {
                is_failing = true;
            }
            continue;            
        }

        is_failing = false;
        result[p] = v;
        stack.push((true, p, v));

        // result[p+1]以降がすべて埋まっていれば探索終了
        let mut is_updated = false;
        for i in p + 1..N{
            if result[i] == 0{
                stack.push((false, i, 1));
                is_updated = true;
                break;
            }
        }
    
        if !is_updated{
            break;
        }
    }

    result.to_vec()
}








////////////////////
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, numberplace!");
}
