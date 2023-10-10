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
    fn test_sum() {
        assert_eq!(super::sum(1, 9), 10);
        assert_eq!(super::sum(-2, 5), 3);
    }
}
