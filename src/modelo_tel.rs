use crate::caracteristicas::Caracteristicas;

pub struct ModeloTel{
    marca: String,
    modelo: String,
    caracteristicas: Caracteristicas,
}

impl ModeloTel{
    pub fn new(new_marca: String,new_modelo: String, new_caracteristicas: Caracteristicas) -> ModeloTel{
        ModeloTel{
            marca:new_marca,
            modelo:new_modelo,
            caracteristicas:new_caracteristicas,
        }
    }

    pub fn get_marca(&self) -> &String {
        &self.marca
    }

    pub fn get_modelo(&self) -> &String {
        &self.modelo
    }

    pub fn get_caracteristicas(&self) -> &Caracteristicas {
        &self.caracteristicas
    }
}
