struct Caracteristicas{
    //las cámaras se expresaran como un entero que contendrá el número de megapixel
    camara_frontal: u16,
    camaras_traseras: Vec<u16>,
    //La capacidad de la memoria se expresará en GB
    ram: u16,
    rom: u16,
    memoria_interna: u16,
    //La velocidad del procesador será expresada en MHz
    vel_procesador: u32,
    //Las dimensiones se expresarán en mm y el orden será siempre altura x anchura x grosor
    dimensiones: [i32; 3],
    //El peso se expresará en gramos
    peso: i32,
    //La capacidad de la batería se indicará en mAh
    capacidad_bateria: u32,
    cinco_g: bool,
    nfc: bool,
    infrarrojos: bool,
    gps: bool,
    lector_huella: bool,
    dual_sim: bool,
    bluetooth: bool,
}