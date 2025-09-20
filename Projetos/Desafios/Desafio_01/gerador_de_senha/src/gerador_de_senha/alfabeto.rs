pub struct Alfabeto {
    alfabeto: Vec<char>
}

impl Alfabeto {
    pub fn new() -> Self {
        let alfabeto: Vec<char> = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
        ];

        Self {
          alfabeto  
        }
    }

    pub fn get_quantidade_de_letras(
        &self
    ) -> usize {
        return self.alfabeto.len();
    }

    pub fn get_alfabeto(
        &self,
        index: usize
    ) -> char {
        return self.alfabeto[
            index
        ];
    }
}