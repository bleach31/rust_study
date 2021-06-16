
pub use typename_derive::TypeName;

pub trait TypeNameTrait {
    fn type_name(&self) -> &str;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
