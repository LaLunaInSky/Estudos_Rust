pub struct Números {
    números: Vec<char>
}

impl Números {
    pub fn new() -> Self {
        let números: Vec<char> = vec![
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
        ];
        
        Self {
            números
        }
    }

    pub fn get_total_de_números(
        &self
    ) -> usize {
        return self.números.len();
    }

    pub fn get_número(
        &self,
        index: usize
    ) -> char {
        return self.números[
            index
        ];
    }
}