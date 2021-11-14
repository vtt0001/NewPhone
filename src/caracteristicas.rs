use std::num;

pub struct Caracteristicas {
    //las cámaras se expresaran como un entero que contendrá el número de megapixel
    camara_frontal: u16,
    camaras_traseras: Vec<u16>,
    //La capacidad de la memoria se expresará en GB
    ram: u16,
    rom: u16,
    memoria_interna: u16,
    //La velocidad del procesador será expresada en MHz
    vel_procesador: f32,
    //Las dimensiones se expresarán en mm y el orden será siempre altura x anchura x grosor
    dimensiones: [f32; 3],
    //El peso se expresará en gramos
    peso: f32,
    //La capacidad de la batería se indicará en mAh
    capacidad_bateria: u32,
    cinco_g: bool,
    nfc: bool,
    infrarrojos: bool,
    gps: bool,
    lector_huella: bool,
    dual_sim: bool,
    bluetooth: bool,
    num_nucleos: u8,
    //El tamaño de la pantalla se indicará en pulgadas
    tam_pantalla: f32,
}

impl Caracteristicas {
    pub fn new(
        new_camara_frontal: u16,
        new_camaras_traseras: Vec<u16>,
        new_ram: u16,
        new_rom: u16,
        new_memoria_interna: u16,
        new_vel_procesador: f32,
        new_dimensiones: [f32; 3],
        new_peso: f32,
        new_capacidad_bateria: u32,
        new_cinco_g: bool,
        new_nfc: bool,
        new_infrarrojos: bool,
        new_gps: bool,
        new_lector_huella: bool,
        new_dual_sim: bool,
        new_bluetooth: bool,
        new_num_nucleos: u8,
        new_tam_pantalla: f32,
    ) -> Caracteristicas {
        Caracteristicas {
            camara_frontal: new_camara_frontal,
            camaras_traseras: new_camaras_traseras,
            ram: new_ram,
            rom: new_rom,
            memoria_interna: new_memoria_interna,
            vel_procesador: new_vel_procesador,
            dimensiones: new_dimensiones,
            peso: new_peso,
            capacidad_bateria: new_capacidad_bateria,
            cinco_g: new_cinco_g,
            nfc: new_nfc,
            infrarrojos: new_infrarrojos,
            gps: new_gps,
            lector_huella: new_lector_huella,
            dual_sim: new_dual_sim,
            bluetooth: new_bluetooth,
            num_nucleos: new_num_nucleos,
            tam_pantalla: new_tam_pantalla,
        }
    }

    pub fn get_camara_frontal(&self) -> &u16 {
        &self.camara_frontal
    }

    pub fn get_camaras_traseras(&self) -> &Vec<u16> {
        &self.camaras_traseras
    }

    pub fn get_ram(&self) -> &u16 {
        &self.ram
    }

    pub fn get_rom(&self) -> &u16 {
        &self.rom
    }

    pub fn get_memoria_interna(&self) -> &u16 {
        &self.memoria_interna
    }

    pub fn get_vel_procesador(&self) -> &f32 {
        &self.vel_procesador
    }

    pub fn get_dimensiones(&self) -> &[f32; 3] {
        &self.dimensiones
    }

    pub fn get_peso(&self) -> &f32 {
        &self.peso
    }

    pub fn get_capacidad_bateria(&self) -> &u32 {
        &self.capacidad_bateria
    }

    pub fn get_cinco_g(&self) -> &bool {
        &self.cinco_g
    }

    pub fn get_nfc(&self) -> &bool {
        &self.nfc
    }

    pub fn get_infrarrojos(&self) -> &bool {
        &self.infrarrojos
    }

    pub fn get_gps(&self) -> &bool {
        &self.gps
    }

    pub fn get_lector_huella(&self) -> &bool {
        &self.lector_huella
    }

    pub fn get_dual_sim(&self) -> &bool {
        &self.dual_sim
    }

    pub fn get_bluetooth(&self) -> &bool {
        &self.bluetooth
    }

    pub fn get_num_nucleos(&self) -> &u8 {
        &self.num_nucleos
    }

    pub fn get_tam_pantalla(&self) -> &f32 {
        &self.tam_pantalla
    }

    //Función que devuelve un vector con los atributos discretizados.
    pub fn discretiza(&self) -> Vec<u8> {
        //Formateando cámaras traseras a un solo valor
        let mut cont: usize = 0;
        let mut sum1: f32 = 1.0;
        let mut sum2: f32 = 0.0;

        while cont < self.camaras_traseras.len() {
            sum1 += 0.1;
            sum2 += self.camaras_traseras[cont] as f32;
            cont += 1;
        }

        //
        let camaras_traseras_in: f32 =
            ((sum2 / self.camaras_traseras.len() as f32) * sum1 * 100.0).round() / 100.0;

        //formateando dimensiones
        let dimensiones_in: f32 =
            ((self.dimensiones[0] * self.dimensiones[1] * self.dimensiones[2] / 1000.0) * 100.0)
                .round()
                / 100.0;

        //Discretizamos valores:
        let camara_frontal: u8 = if self.camara_frontal <= 8 {
            0
        } else if self.camara_frontal > 8 && self.camara_frontal <= 12 {
            1
        } else {
            2
        };
        let camaras_traseras: u8 = if camaras_traseras_in <= 30.0 {
            0
        } else if camaras_traseras_in > 30.0 && camaras_traseras_in <= 40.0 {
            1
        } else {
            2
        };
        let ram: u8 = if self.ram <= 4 {
            0
        } else if self.ram > 4 && self.ram <= 8 {
            1
        } else {
            2
        };
        let rom: u8 = if self.rom <= 32 {
            0
        } else if self.rom > 32 && self.rom <= 64 {
            1
        } else {
            2
        };
        let memoria_interna: u8 = if self.memoria_interna <= 256 {
            0
        } else if self.memoria_interna > 256 && self.memoria_interna <= 512 {
            1
        } else {
            2
        };
        let vel_procesador: u8 = if self.vel_procesador <= 1.2 {
            0
        } else if self.vel_procesador > 1.2 && self.vel_procesador <= 2.4 {
            1
        } else {
            2
        };
        let dimensiones: u8 = if dimensiones_in <= 85.0 {
            0
        } else if dimensiones_in > 85.0 && dimensiones_in <= 100.0 {
            1
        } else {
            2
        };
        let peso: u8 = if self.peso <= 180.0 {
            0
        } else if self.peso > 180.0 && self.peso <= 250.0 {
            1
        } else {
            2
        };
        let capacidad_bateria: u8 = if self.capacidad_bateria <= 3000 {
            0
        } else if self.capacidad_bateria > 3000 && self.capacidad_bateria <= 4500 {
            1
        } else {
            2
        };
        let cinco_g: u8 = if self.cinco_g { 1 } else { 0 };
        let nfc: u8 = if self.nfc { 1 } else { 0 };
        let infrarrojos: u8 = if self.infrarrojos { 1 } else { 0 };
        let gps: u8 = if self.gps { 1 } else { 0 };
        let lector_huella: u8 = if self.lector_huella { 1 } else { 0 };
        let dual_sim: u8 = if self.dual_sim { 1 } else { 0 };
        let bluetooth: u8 = if self.bluetooth { 1 } else { 0 };
        let num_nucleos: u8 = if self.num_nucleos <= 2 {
            0
        } else if self.num_nucleos > 2 && self.num_nucleos <= 6 {
            1
        } else {
            2
        };
        let tam_pantalla: u8 = if self.tam_pantalla <= 5.0 {
            0
        } else if self.tam_pantalla > 5.0 && self.tam_pantalla <= 5.5 {
            1
        } else {
            2
        };

        let mut vector: Vec<u8> = vec![
            camara_frontal,
            camaras_traseras,
            ram,
            rom,
            memoria_interna,
            vel_procesador,
            dimensiones,
            peso,
            capacidad_bateria,
            cinco_g,
            nfc,
            infrarrojos,
            gps,
            lector_huella,
            dual_sim,
            bluetooth,
            num_nucleos,
            tam_pantalla,
        ];

        vector
    }
}
