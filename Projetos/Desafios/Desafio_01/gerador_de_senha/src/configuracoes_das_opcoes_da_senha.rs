pub struct ConfiguraçãoDasOpções {
    contém_números: bool,
    contém_símbolos: bool,
    contém_maiúsculas: bool,
    total_de_letras: u8
}

impl ConfiguraçãoDasOpções {
    pub fn new() -> Self {
        Self {
            contém_números: false,
            contém_símbolos: false,
            contém_maiúsculas: false,
            total_de_letras: 8
        }
    }

    fn analisar_o_bool(
        &self,
        bool: bool
    ) -> String {
        match bool {
            true => return String::from("SIM"),
            false => return String::from("NÃO")
        }
    }

    pub fn set_total_de_letras(
        &mut self,
        total_de_letras: u8
    ) {
        self.total_de_letras = total_de_letras;
    }

    pub fn set_contém_números(
        &mut self
    ) {
        if self.contém_números {
            self.contém_números = false;
        } else {
            self.contém_números = true;
        }
    }

    pub fn set_contém_símbolos(
        &mut self
    ) {
        if self.contém_símbolos {
            self.contém_símbolos = false;
        } else {
            self.contém_símbolos = true;
        }
    }

    pub fn set_contém_maiúsculas(
        &mut self
    ) {
        if self.contém_maiúsculas {
            self.contém_maiúsculas = false;
        } else {
            self.contém_maiúsculas = true;
        }
    }

    pub fn get_contém_números(
        &self
    ) -> String {
        return self.analisar_o_bool(
            self.contém_números
        );
    }

    pub fn get_contém_símbolos(
        &self
    ) -> String {
        return self.analisar_o_bool(
            self.contém_símbolos
        );
    }

    pub fn get_contém_maiúsculas(
        &self
    ) -> String {
        return self.analisar_o_bool(
            self.contém_maiúsculas
        );
    }

    pub fn get_total_de_letras(
        &self
    ) -> u8 {
        return self.total_de_letras;
    }

    pub fn get_mostrador_de_opções(
        &self
    ) {
        println!(
            "
 1. [ {:^3} ] - Números
 2. [ {:^3} ] - Símbolos
 3. [ {:^3} ] - Maiúsculas
 4. [{:^5}] - Total de letras
 5. Gerar a Senha
 6. Fechar Programa
        ",
            self.get_contém_números(),
            self.get_contém_símbolos(),
            self.get_contém_maiúsculas(),
            self.get_total_de_letras()
        ); 
    }
}