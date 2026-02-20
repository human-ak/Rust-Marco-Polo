/* Creata a test for marco polo game */

#[cfg(test)]
mod tests {

    #[test]
    fn test_marco_polo() {
        assert_eq!(marco::marco_polo("Marco"), "Polo");
        assert_eq!(marco::marco_polo("Alice"), "What's ur name?");
    }
}