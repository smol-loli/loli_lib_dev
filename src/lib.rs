pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod lstd {
    /// Prints out my reaction when my code finally works xD
    ///
    /// # Example
    ///
    /// Eeeeeh? You really expected a proper example here?
    /// Are you out of your mind? It's version v0.0.1 what did you expect?!?
    ///

    pub fn test_println() {
        println!("Yay, it works!");
    }
}


#[cfg(test)]// 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
