pub mod species;
mod msg;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn new_fn () {
    let mi_var : String = String::from("myvar");
    let xx : i64 = 32;
    format!("{}",xx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
