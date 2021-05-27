macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

macro_rules! vec1 {
    ($x:expr) => {{
        let mut temp_vec = Vec::new();
        temp_vec.push($x);
        temp_vec
    }};
}

macro_rules! vec2 {
    ($($x:expr),*) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}

macro_rules! vec3 { 
    ($x:ty) => {
        {
            let temp_vec: Vec<$x> = Vec::new();
            temp_vec
        }
    };
    
    ($($x:expr),*) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}


fn main() {
    println!("Hello, world!");

    assert_eq!(25,five_times![2+3]);

    let x = vec1!(0);
    println!("{:?}", x);

    let x = vec2!(0,1,2);
    println!("{:?}", x);

    let x = vec3!(i32);
    println!("{:?}", x);

    let mut x = vec2!();
    x.push(100);
    println!("{:?}", x);

}

