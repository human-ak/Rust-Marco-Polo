/* write unit tests for src/lib.rs */

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_add() {
        assert_eq!(calc::add(2.0, 3.0), 5.0);
        assert_eq!(calc::add(-1.0, 1.0), 0.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(calc::subtract(5.0, 3.0), 2.0);
        assert_eq!(calc::subtract(0.0, 1.0), -1.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(calc::multiply(2.0, 3.0), 6.0);
        assert_eq!(calc::multiply(-1.0, 1.0), -1.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(calc::divide(6.0, 3.0), 2.0);
        assert_eq!(calc::divide(-1.0, 1.0), -1.0);
    }
}
