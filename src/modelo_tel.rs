use crate::caracteristicas::Caracteristicas;
use std::num;

const MAX_DISTANCIA: f64 = 7.1414284285428504;
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

    pub fn compare_modelo(&self, form_usuario: Vec<u8> ) -> f64{

        let vec_modelo: Vec<u8> = self.caracteristicas.discretiza();

        let mut count: usize = 0;
        let mut sum: isize = 0;
        
        while count < 18{
            let x = (vec_modelo[count] as isize -form_usuario[count] as isize) * (vec_modelo[count] as isize-form_usuario[count] as isize);
            sum += x;
            count += 1;
        }
        (sum as f64).sqrt() / MAX_DISTANCIA
    }
}