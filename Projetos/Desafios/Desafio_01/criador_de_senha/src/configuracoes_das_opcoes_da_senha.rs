pub struct ConfiguraçãoDasOpções {
    contém_números: bool,
    contém_símbolos: bool,
    contém_maiúsculas: bool,
}

impl ConfiguraçãoDasOpções {
    pub fn new() -> Self {
        Self {
            contém_números: false,
            contém_símbolos: false,
            contém_maiúsculas: false,
        }
    }

    fn analisar_o_bool(
        &self,
        bool: bool
    ) -> String {
        match bool {
            true => return String::from("ON"),
            false => return String::from("OFF")
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
}