mod socialism;

#[cfg(test)]
mod tests {
    use socialism::redistribute;
    #[test]
    fn test_1_star_1() {
        assert_eq!(redistribute("0 2 7 0"), 5);
    }
    #[test]
    fn star_1_for_real() {
        assert_eq!(redistribute("10 3   15  10  5   15  5   15  9   2   5   8   5   2   3   6"), 14029);
    }
}
