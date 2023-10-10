fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn main() {
    let c: u32 = sum(2, 3);
    println!("result: {}", c);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum_1() {
        assert_eq!(super::sum(2, 3), 5);
        assert_eq!(super::sum(2, 5), 7);
    }

    #[test]
    fn test_sum_2() {
        assert_eq!(super::sum(1, 10), 11);
        assert_eq!(super::sum(0, 3), 3);
    }
}
