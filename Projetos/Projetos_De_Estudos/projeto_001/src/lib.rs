// Crie um programa que leia dois números e mostre a soma entre eles.

use crate::soma_de_dois_números_inteiros;

pub fn soma_de_dois_números_inteiros(primeiro_número: u32, segundo_número: u32) -> u32 {
    primeiro_número + segundo_número 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soma_dos_dois_números_funciona() {
        let primeiro_número: u32 = 5;
        let segundo_número: u32 = 5;

        let resultado_da_soma_dos_dois_números: u32 = soma_de_dois_números_inteiros(primeiro_número, segundo_número);
    
        assert_eq!(resultado_da_soma_dos_dois_números, 10);
    }
}