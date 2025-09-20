pub struct Símbolos {
    símbolos: Vec<char>
}

impl Símbolos {
    pub fn new() -> Self {
        let símbolos: Vec<char> = vec![
            '!', '@', '#', '$', '%', '&', '*', '(', ')', '_', '-', '+', '=', '§', '[', ']', '{', '}', '?', '/'
        ];

        Self {
            símbolos
        }
    }

    pub fn get_quantidade_de_símbolos(
        &self
    ) -> usize {
        return self.símbolos.len();
    }

    pub fn get_símbolo(
        &self,
        index: usize
    ) -> char {
        return self.símbolos[
            index
        ];
    }
}