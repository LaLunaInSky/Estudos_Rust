pub struct AlfabetoMaiúsculo {
    alfabeto_maiúsculo: Vec<char>
}

impl AlfabetoMaiúsculo {
    pub fn new() -> Self {
        let alfabeto_maiúsculo: Vec<char> = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
        ];

        Self {
          alfabeto_maiúsculo  
        }
    }

    pub fn get_quantidade_de_letras(
        &self
    ) -> usize {
        return self.alfabeto_maiúsculo.len();
    }

    pub fn get_alfabeto_maiúsculo(
        &self,
        index: usize
    ) -> char {
        return self.alfabeto_maiúsculo[
            index
        ];
    }
}