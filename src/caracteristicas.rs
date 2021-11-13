pub struct Caracteristicas{
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

impl Caracteristicas{
    pub fn new(new_camara_frontal: u16, new_camaras_traseras: Vec<u16>, new_ram: u16, new_rom: u16, new_memoria_interna: u16, new_vel_procesador: f32, new_dimensiones: [f32; 3], new_peso: f32, new_capacidad_bateria: u32, new_cinco_g: bool, new_nfc: bool, new_infrarrojos: bool, new_gps: bool, new_lector_huella: bool, new_dual_sim: bool, new_bluetooth: bool, new_num_nucleos: u8, new_tam_pantalla: f32,) -> Caracteristicas{
        Caracteristicas{
            camara_frontal:new_camara_frontal,
            camaras_traseras:new_camaras_traseras,
            ram:new_ram,
            rom:new_rom,
            memoria_interna:new_memoria_interna,
            vel_procesador:new_vel_procesador,
            dimensiones:new_dimensiones,
            peso:new_peso,
            capacidad_bateria:new_capacidad_bateria,
            cinco_g:new_cinco_g,
            nfc:new_nfc,
            infrarrojos:new_infrarrojos,
            gps:new_gps,
            lector_huella:new_lector_huella,
            dual_sim:new_dual_sim,
            bluetooth:new_bluetooth,
            num_nucleos:new_num_nucleos,
            tam_pantalla:new_tam_pantalla,
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

    pub fn get_peso(&self) -> &f32{
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
}
