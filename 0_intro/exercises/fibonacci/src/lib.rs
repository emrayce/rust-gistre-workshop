pub fn fibo(v: u32) -> u32 {
    let mut f0: u32 = 0;
    let mut f1: u32 = 1;

    for i in 1..=v {
        let mut tmp: u32 = f0;
        f0 = f1;
        f1 = tmp + f0;
    }

    f0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibo_0() {
        assert_eq!(0, fibo(0));
    }

    #[test]
    fn fibo_1() {
        assert_eq!(1, fibo(1));
    }

    #[test]
    fn fibo_2() {
        assert_eq!(1, fibo(2));
    }

    #[test]
    fn fibo_big() {
        assert_eq!(610, fibo(15));
    }
}
