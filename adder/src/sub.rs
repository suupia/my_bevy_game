
mod sub{
    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_subs_two() {
        assert_eq!(4, sub::sub(7,3));
    }

}

