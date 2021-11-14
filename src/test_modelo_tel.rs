
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

     // Test que verifica si calcula de forma correcta los valores discretizados (HU1, #20)
    #[test]
    fn test_discretiza_formatea_funcionalidad(){
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

        let mut vec_disc: Vec<u8>= caract.discretiza();
        
        let esperados: Vec<u8> = vec![1,2,1,2,0,2,2,1,1,0,1,1,1,0,0,1,1,2];
        let mut count: usize = 0;
        
        while count < 18 {
            let x = vec_disc[count];
            assert_that!(&vec_disc[count], eq(esperados[count]));
            count += 1;
        }
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

    //Comprueba que la distancia euclidiana calculada en el método compare_modelo es 0 cuando el parámetro vector y los atributos discretizados 
    //tienen los mismos valores. (HU1 #9)
    #[test]
    fn test_compare_iguales(){
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

        let form_usuario: Vec<u8> = vec![1,2,1,2,0,2,2,1,1,0,1,1,1,0,0,1,1,2];

        assert_that!(&movil.compare_modelo(form_usuario), eq(0.0));
    }

    //Comprueba que la distancia euclidiana calculada en el método compare_modelo es 1 cuando el parámetro vector y los atributos discretizados 
    //tienen valores completamente opuestos. (HU1 #9)
    #[test]
    fn test_compare_opuestos(){
        let camara_frontal          : u16         = 6;
        let ram                     : u16         = 4;
        let rom                     : u16         = 16;
        let vel_procesador          : f32         = 1.2;
        let dimensiones             : [f32;3]     =[60.0, 102.5, 6.0];
        let peso                    : f32         = 175.5;
        let capacidad_bateria       : u32         = 3000;
        let cinco_g                 : bool        = false;
        let nfc                     : bool        = false;
        let infrarrojos             : bool        = false;
        let gps                     : bool        = false;
        let lector_huella           : bool        = false;
        let dual_sim                : bool        = false;
        let bluetooth               : bool        = false;
        let num_nucleos             : u8          = 2;
        let tam_pantalla            : f32         = 5.0;
        let memoria_interna         :u16          = 128;   
        let mut camaras_traseras        : Vec<u16>    = Vec::new();
        camaras_traseras.push(12);

        let caract: Caracteristicas = Caracteristicas::new(camara_frontal, camaras_traseras, ram, rom, 
            memoria_interna, vel_procesador, dimensiones, peso, 
            capacidad_bateria, cinco_g, nfc, infrarrojos, gps,
        lector_huella, dual_sim, bluetooth, num_nucleos, tam_pantalla);

        let marca: String = "Samsung".to_string();
        let modelo: String = "Galaxy Note 10".to_string();
        let movil: ModeloTel = ModeloTel::new(marca, modelo, caract);

        let form_usuario: Vec<u8> = vec![2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,2,2];

        let x = movil.compare_modelo(form_usuario.clone());
        assert_that!(&movil.compare_modelo(form_usuario), eq(1.0));
    }
}

