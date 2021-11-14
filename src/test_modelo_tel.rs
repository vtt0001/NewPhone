
mod test_caracteristicas_galvanic{

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
        assert_that!(&caract.get_camara_frontal(), eq(12));
        assert_that!(&caract.get_camaras_traseras()[0], eq(108));
        assert_that!(&caract.get_ram(), eq(8));
        assert_that!(&caract.get_rom(), eq(128));
        assert_that!(&caract.get_vel_procesador(), eq(2.84));
        assert_that!(&caract.get_dimensiones(), eq([74.8, 162.6, 9.0]));
        assert_that!(&caract.get_peso(), eq(208.5));
        assert_that!(&caract.get_capacidad_bateria(), eq(4500));
        assert_that!(&caract.get_cinco_g(), eq(false));
        assert_that!(&caract.get_nfc(), eq(true));
        assert_that!(&caract.get_infrarrojos(), eq(true));
        assert_that!(&caract.get_gps(), eq(true));
        assert_that!(&caract.get_lector_huella(), eq(false));
        assert_that!(&caract.get_dual_sim(), eq(false));
        assert_that!(&caract.get_bluetooth(), eq(true));
        assert_that!(&caract.get_num_nucleos(), eq(4));
        assert_that!(&caract.get_tam_pantalla(), eq(6.5));
        assert_that!(&caract.get_memoria_interna(), eq(128));
    }

     // Test que verifica si el constructor funciona correctamente (HU1, #18)
    #[test]
    fn test_formaliza_discretiza(){
        let camara_frontal          : u16         = 12;
        let ram                     : u16         = 8;
        let rom                     : u16         = 128;
        let vel_procesador          : f32         = 2.84;
        let dimensiones             : [f32;3]     =[75.8, 162.4, 9.2];
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

        let tupla_form_disc: (f32, f32, u8, u8, u8, u8, u8, u8, u8) = caract.discretiza_normaliza();
        
        assert_that!(&tupla_form_disc.0, eq(43.75));
        assert_that!(&tupla_form_disc.1, eq(113251.26));
        assert_that!(&tupla_form_disc.2, eq(0));
        assert_that!(&tupla_form_disc.3, eq(1));
        assert_that!(&tupla_form_disc.4, eq(1));
        assert_that!(&tupla_form_disc.5, eq(1));
        assert_that!(&tupla_form_disc.6, eq(0));
        assert_that!(&tupla_form_disc.7, eq(0));
        assert_that!(&tupla_form_disc.8, eq(1));
    }
}

mod test_modelo_tel_galvanic{

    use galvanic_assert::*;
    use galvanic_assert::matchers::*;
    use crate::caracteristicas::Caracteristicas;
    use crate::modelo_tel::ModeloTel;

    // Test que verifica si el constructor funciona correctamente (HU1, #19)
    #[test]
    fn test_nuevo_modelotel(){
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

        let marca: String = "Samsung".to_string();
        let modelo: String = "Galaxy Note 10".to_string();
        let movil: ModeloTel = ModeloTel::new(marca, modelo, caract);

        assert_that!(&movil, is_variant!(ModeloTel));
        assert_that!(&movil.get_marca(), eq("Samsung".to_string()));
        assert_that!(&movil.get_modelo(), eq("Galaxy Note 10".to_string()));
    }
}

