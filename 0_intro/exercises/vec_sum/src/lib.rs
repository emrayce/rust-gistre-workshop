pub fn vec_sum(vec: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in vec.iter() {
        sum += i;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_sum_3() {
        assert_eq!(3, vec_sum(vec![1, 2]));
    }

    #[test]
    fn vec_sum_10() {
        assert_eq!(10, vec_sum(vec![2, 3, 5]));
    }
}
