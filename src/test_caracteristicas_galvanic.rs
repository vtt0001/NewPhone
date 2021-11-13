
mod test_caracteristicas_galvanic{

    //#[macro_use]
    //extern crate galvanic_assert;
    //use galvanic_assert::matchers::*;
    use galvanic_assert::*;
    use galvanic_assert::matchers::*;
    use crate::caracteristicas::Caracteristicas;

    // Test que verifica si el constructor funciona correctamente (HU1, #17)
    #[test]
    fn test_nuevas_caracteristicas(){
        let camara_frontal          : u16         = 12;
        let ram                     : u16         = 8;
        let rom                     : u16         = 128;
        let vel_procesador          : f32         = 2.84;
        let dimensiones             : [f32;3]     =[74.8, 162.6, 9.0];
        let peso                    : f32         = 208.5;
        let capacidad_bateria       : u32         = 4500;
        let cinco_g                 : bool        = false;
        let nfc                     : bool        = true;
        let infrarrojos             : bool        = true;
        let gps                     : bool        = true;
        let lector_huella           : bool        = false;
        let dual_sim                : bool        = false;
        let bluetooth               : bool        = true;
        let num_nucleos             : u8          = 4;
        let tam_pantalla            : f32         = 6.5;
        let memoria_interna         :u16          = 128;   
        let mut camaras_traseras        : Vec<u16>    = Vec::new();
        camaras_traseras.push(108);
        camaras_traseras.push(13);
        camaras_traseras.push(2);
        camaras_traseras.push(2);

        let caract: Caracteristicas = Caracteristicas::new(camara_frontal, camaras_traseras, ram, rom, 
            memoria_interna, vel_procesador, dimensiones, peso, 
            capacidad_bateria, cinco_g, nfc, infrarrojos, gps,
        lector_huella, dual_sim, bluetooth, num_nucleos, tam_pantalla);

        assert_that!(&caract, is_variant!(Caracteristicas));
    }

    /*#[test]
    fn expression_should_compute_correct_value() {
        assert_that!(&3, eq(3));
    }*/
}
