/* A Marco Polo game. */
/*
If the name Marco is given, the program will print Polo,
else it prints "What's ur name?".
*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What's ur name?".to_string()
    }
}
