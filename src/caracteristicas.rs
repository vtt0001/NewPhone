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
    pub fn new(new_camara_frontal: u16, new_camaras_traseras: Vec<u16>, new_ram: u16, new_rom: u16, 
        new_memoria_interna: u16, vel_procesador: f32, new_dimensiones: [f32; 3], new_peso: f32, 
        new_capacidad_bateria: u32, new_cinco_g: bool, new_nfc: bool, new_infrarrojos: bool, new_gps: bool,
    new_lector_huella: bool, new_dual_sim: bool, new_bluetooth: bool, new_num_nucleos: u8, new_tam_pantalla: f32,) -> Caracteristicas{
        panic!();
    }
}
