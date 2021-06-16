use proc_macro::{TokenStream,Group,TokenTree,Delimiter};
use std::str::FromStr;

#[proc_macro]
pub fn tomlstruct(input: TokenStream) -> TokenStream {
    //dbg!(&input);
    let mut ret = String::from(""); //fromて何

    for token in input {
        match &token {
            //[hello]
            TokenTree::Group(x) => {
                let name = get_struct_name(x).unwrap();
                if ret == "" { //なにこのかっこ
                    ret = format!("struct {} {{", name);
                }else{
                    ret = format!("{}}}\nstruct {}{{",ret,name);
                }
            }
            //name, version
            TokenTree::Ident(x) => {
                ret = format!("{}\n    {}",ret,x.to_string());
            }
            //"hello", 1.0
            TokenTree::Literal(x) => {
                //println!("{:?}",x);
                //println!("{}",x.to_string());
                if x.to_string().starts_with('"'){
                    ret = format!("{}: String,",ret);
                }else{
                    ret = format!("{}: f64,",ret);
                }
            }
            _ => (),
        }
    }
    ret = format!("{}\n}}",ret);
    //println!("{}",&ret);
    FromStr::from_str(&ret).unwrap()
}

fn get_struct_name(input: &Group) -> Option<String>{
    match input.delimiter(){
        Delimiter::Bracket => { //Bracketは[ ]のこと
            for token in input.stream(){
                if let TokenTree::Ident(x) = token { //なんで代入してんの？ if let構文、パターンパッチの簡易版
                    return Some(x.to_string());
                }
            }
        }
        _ => (),
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
