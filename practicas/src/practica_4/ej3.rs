/*
    3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
    (Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una duración
    de meses y una fecha de inicio, además los usuarios pueden pagar por sus suscripciones con
    distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de Crédito, Transferencia
    Bancaria, Cripto. Cada medio de pago tiene sus datos correspondientes a excepción de Efectivo.
    Los usuarios solo pueden tener una suscripción activa a la vez.
    Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
    siguientes acciones:
        ➢ Crear un usuario con una determinada suscripción y medio de pago.
        ➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
        pasa a Clasic y si está en Clasic pasa a Super.
        ➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
        suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
        ➢ Dado un usuario cancelar la suscripción.
        ➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
        suscripciones activas
        ➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
        activas.
        ➢ Saber cuál fue el medio de pago más utilizado.
        ➢ Saber cuál fue la suscripción más contratada.
*/
pub fn ejercicio3() { 
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "classic".to_string(), "efectivo".to_string());
    let usuario_2:Usuario = Usuario::new("Ema".to_string(), "super".to_string(), "tarjeta de credito".to_string());
    let usuario_3:Usuario = Usuario::new("Oca".to_string(), "basic".to_string(), "cripto".to_string());
    let csm:StreamingRust = StreamingRust { clientes: vec![usuario_1, usuario_2, usuario_3] } ;
}
//mock
#[derive(Debug)]
#[derive(PartialEq)]
struct FechaHoy{
    dia:u8,
    mes:u8,
    año:u16,
}
impl FechaHoy{
    pub fn new() -> FechaHoy {
        FechaHoy { dia: 1, mes: 1, año: 1901 }
    }
}
//mock
#[derive(Debug)]
#[derive(PartialEq)]
struct MedioDePago {
    medio_de_pago: Pago,
    datos: Option<String>
}
impl MedioDePago {
    pub fn new(medio:&String) -> MedioDePago {
        let mp:String = medio.to_lowercase();
        if mp == "efectivo".to_string() {
            MedioDePago { medio_de_pago : Pago::Efectivo, datos : None }
        }else{
            match mp.as_str(){
                "cripto" => MedioDePago { medio_de_pago : Pago::Cripto, datos : Some("Datos ~ Cripto".to_string()) },
                "mercadopago" => MedioDePago { medio_de_pago : Pago::MercadoPago, datos : Some("Datos ~ MercadoPago".to_string()) },
                "tarjeta de credito" => MedioDePago { medio_de_pago : Pago::TarjetaCredito, datos : Some("Datos ~ Tarjeta de Crédito".to_string()) },
                "transferencia" => MedioDePago { medio_de_pago : Pago::Transferencia, datos : Some("Datos ~ Transferencia".to_string()) },
                _ => panic!("Medio de pago invalido.")
            }
        }
    }
}
#[derive(Debug)]
#[derive(PartialEq)]
enum Pago {
    Efectivo,
    MercadoPago,
    TarjetaCredito,
    Transferencia,
    Cripto,
}
#[derive(Debug)]
#[derive(PartialEq)]
enum TipoSubscripcion{
    Basic,
    Classic,
    Super,
}
impl TipoSubscripcion{
    pub fn mi_subscripcion(&self) -> TipoSubscripcion {
        match self{
            TipoSubscripcion::Basic => TipoSubscripcion::Basic,
            TipoSubscripcion::Classic => TipoSubscripcion::Classic,
            TipoSubscripcion::Super => TipoSubscripcion::Super,
        }
    }
}
#[derive(Debug)]
#[derive(PartialEq)]
struct Subscripcion{
    tipo: TipoSubscripcion,
    costo: f64,
    duracion: u8,
    inicio: FechaHoy,
}
impl Subscripcion{
    pub fn new(tipo:&String) -> Subscripcion {
        let s:String = tipo.to_lowercase();
        match s.as_str() {
            "basic" => Subscripcion { tipo: TipoSubscripcion::Basic, costo:1000.0, duracion:1, 
                        inicio: FechaHoy::new() },
            "classic" => Subscripcion { tipo: TipoSubscripcion::Classic, costo: 3000.0, duracion:2,
                            inicio: FechaHoy::new() },
            "super" => Subscripcion { tipo: TipoSubscripcion::Super, costo: 5000.0, duracion: 3,
                            inicio: FechaHoy::new() },
            &_ => panic!("Tipo de subscripción invalida.")
        }
    }
}
#[derive(Debug)]
struct Usuario{
    nombre: String,
    subscripcion: Subscripcion,
    medio_de_pago: MedioDePago,
    activo: bool,
}
impl Usuario{
    pub fn new(nom:String, sub:String, mp:String) -> Usuario {
        Usuario {
            nombre: nom,
            subscripcion: Subscripcion::new(&sub),
            medio_de_pago: MedioDePago::new(&mp),
            activo: true
        }
    }
    pub fn cancelar_subscripcion(&mut self){
        self.activo = false;
    }
    pub fn update(&mut self){
        match self.subscripcion.tipo.mi_subscripcion(){
            TipoSubscripcion::Basic => self.subscripcion = Subscripcion::new(&"Classic".to_string()),
            TipoSubscripcion::Classic => self.subscripcion = Subscripcion::new(&"Super".to_string()),
            _  => panic!("No se puede upgradear si tu tipo de subscripción es Super.")
        }
    }
    pub fn downgrade(&mut self){
        match self.subscripcion.tipo.mi_subscripcion(){
            TipoSubscripcion::Basic => self.cancelar_subscripcion(),
            TipoSubscripcion::Classic => self.subscripcion = Subscripcion::new(&"Basic".to_string()),
            TipoSubscripcion::Super => self.subscripcion = Subscripcion::new(&"Classic".to_string()),
        }
    }        
}
#[derive(Debug)]
struct StreamingRust{
    clientes: Vec<Usuario>
}
impl StreamingRust{
    pub fn buscar_usuario(&self, nombre_de_usuario:&String) -> Option<&Usuario> {
        return self.clientes.iter().find(|&c| c.nombre.eq(nombre_de_usuario));
    }

    pub fn buscar_usuario_mut(&mut self, nombre_de_usuario:&String) -> Option<&mut Usuario> {
        return self.clientes.iter_mut().find(|c| c.nombre.eq(nombre_de_usuario));
    }
    
    // ➢ Crear un usuario con una determinada suscripción y medio de pago.
    pub fn crear_usuario(&mut self, nom:String, sub:String, mp:String) -> bool{
        if let Some(usuario_existe) = self.buscar_usuario(&nom) {
            return false;
        }else{
            self.clientes.push(Usuario::new(nom, sub, mp));
            return true;
        }
    }
    
    // ➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic y si está en Clasic pasa a Super.
    pub fn updatear_usuario(&mut self, usu:&String) -> bool {
        match self.buscar_usuario_mut(usu){
            None => return false,
            Some(u) => { u.update(); return true; }
        }
    }
    
    // ➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
    pub fn downgradear_usuario(&mut self, usu:&String) -> bool {
        match self.buscar_usuario_mut(usu){
            None => return false,
            Some(u) => {u.downgrade(); return true;}
        }
    }
    
    // ➢ Dado un usuario cancelar la suscripción.
    pub fn cancelar_subscripcion(&mut self, usu:&String) -> bool {
        match self.buscar_usuario_mut(usu){
            None => return false,
            Some(u) => {u.cancelar_subscripcion(); return true;}
        }
    }
    
    pub fn contar_subscripciones(&self, p:&Pago) -> u32 {
        return self.clientes.iter()
        .filter(|&c| c.activo == true && c.medio_de_pago.medio_de_pago == *p)
        .count() as u32;
    }

    // ➢ Saber el medio de pago que es más utilizado por los usuarios sobre las suscripciones activas
    pub fn top_active_payment(&self) -> String {
        let cant_efectivos:u32 = self.contar_subscripciones(&Pago::Efectivo);
        let cant_cripto:u32 = self.contar_subscripciones(&Pago::Cripto);
        let cant_mp:u32 = self.contar_subscripciones(&Pago::MercadoPago);
        let cant_trans:u32 = self.contar_subscripciones(&Pago::Transferencia);
        let cant_tarjeta:u32 = self.contar_subscripciones(&Pago::TarjetaCredito);
        
        let medios_de_pago: Vec<u32> = vec![cant_cripto, cant_efectivos, cant_mp, cant_tarjeta, cant_trans];
        
        let mut max:u32 = 0;
        let mut indice_max: i8 = -1;
        for i in 0..medios_de_pago.len()-1 {
            if medios_de_pago[i] > max {
                max = medios_de_pago[i];
                indice_max = i as i8;
            }
        }
        
        match indice_max {
            1 => return "Cripto".to_string(),
            2 => return "Efectivo".to_string(),
            3 => return "Mercado Pago".to_string(),
            4 => return "Tarjeta".to_string(),
            5 => return "Transferencia".to_string(),
            _ => panic!("")
        }

        //ver sort by value hashmap 
    }
}

//➢ Crear un usuario con una determinada suscripción y medio de pago.
#[test]
fn test_crear_usuario(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "classic".to_string(), "efectivo".to_string());
    let usuario_2:Usuario = Usuario::new("Ema".to_string(), "super".to_string(), "tarjeta de credito".to_string());
    let usuario_3:Usuario = Usuario::new("Oca".to_string(), "basic".to_string(), "cripto".to_string());
    
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1, usuario_2, usuario_3] } ;

    assert_eq!(csr.clientes.last().unwrap().nombre, "Oca".to_string());
    assert_eq!(csr.clientes.len(), 3);

    assert_eq!(csr.crear_usuario("Dante".to_string(), "classic".to_string(), "efectivo".to_string()), true);

    assert_eq!(csr.clientes.last().unwrap().nombre, "Dante".to_string());
    assert_eq!(csr.clientes.len(), 4);
}
#[test]
fn test_crear_usuario_to_lower(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "CLASSIC".to_string(), "efectivo".to_string());
    let usuario_2:Usuario = Usuario::new("Oca".to_string(), "basic".to_string(), "CRIPTO".to_string());
    
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1, usuario_2] } ;

    assert_eq!(csr.clientes.last().unwrap().nombre, "Oca".to_string());
    assert_eq!(csr.clientes.len(), 2);

    assert_eq!(csr.crear_usuario("Dante".to_string(), "CLAssic".to_string(), "efeCtivo".to_string()), true);

    assert_eq!(csr.clientes.last().unwrap().nombre, "Dante".to_string());
    assert_eq!(csr.clientes.len(), 3);
}
#[test]
fn test_crear_usuario_nombre_repetido(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "classic".to_string(), "efectivo".to_string());
    let usuario_2:Usuario = Usuario::new("Ema".to_string(), "super".to_string(), "tarjeta de credito".to_string());
    let usuario_3:Usuario = Usuario::new("Oca".to_string(), "basic".to_string(), "cripto".to_string());
    
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1, usuario_2, usuario_3] } ;

    assert_eq!(csr.clientes.last().unwrap().nombre, "Oca".to_string());
    assert_eq!(csr.clientes.len(), 3);

    assert_eq!(csr.crear_usuario("Ana".to_string(), "classic".to_string(), "efectivo".to_string()), false);
    assert_eq!(csr.clientes.last().unwrap().nombre, "Oca".to_string());
    assert_eq!(csr.clientes.len(), 3);
}
#[test]
#[should_panic(expected = "Medio de pago invalido.")]
fn test_crear_usuario_panic_mp_invalido(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "classic".to_string(), "efectivo".to_string());
    let usuario_2:Usuario = Usuario::new("Ema".to_string(), "super".to_string(), "tarjeta de credito".to_string());
    let usuario_3:Usuario = Usuario::new("Oca".to_string(), "basic".to_string(), "cripto".to_string());
    
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1, usuario_2, usuario_3] } ;

    csr.crear_usuario("Manu".to_string(), "classic".to_string(), "qwerty".to_string());
}
#[test]
#[should_panic(expected = "Tipo de subscripción invalida.")]
fn test_crear_usuario_panic_sub_invalida(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "classic".to_string(), "efectivo".to_string());
    let usuario_2:Usuario = Usuario::new("Ema".to_string(), "super".to_string(), "tarjeta de credito".to_string());
    let usuario_3:Usuario = Usuario::new("Oca".to_string(), "basic".to_string(), "cripto".to_string());
    
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1, usuario_2, usuario_3] } ;

    csr.crear_usuario("Manu".to_string(), "qwerty".to_string(), "efectivos".to_string());
}
//➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic 
//y si está en Clasic pasa a Super.
#[test]
fn test_updatear_usuario_basic_a_classic(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "BASIC".to_string(), "efectivo".to_string());
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1] } ;

    assert_eq!(csr.clientes[0].subscripcion.tipo, TipoSubscripcion::Basic);

    csr.updatear_usuario(&"Ana".to_string());

    assert_eq!(csr.clientes[0].subscripcion.tipo, TipoSubscripcion::Classic);
}
#[test]
fn test_updatear_usuario_classic_a_super(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "CLASSIC".to_string(), "efectivo".to_string());
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1] } ;

    assert_eq!(csr.clientes[0].subscripcion.tipo, TipoSubscripcion::Classic);

    csr.updatear_usuario(&"Ana".to_string());

    assert_eq!(csr.clientes[0].subscripcion.tipo, TipoSubscripcion::Super);
}
#[test]
#[should_panic(expected = "No se puede upgradear si tu tipo de subscripción es Super.")]
fn test_updatear_usuario_panic_super(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "SUPER".to_string(), "efectivo".to_string());
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1] } ;
    csr.updatear_usuario(&"Ana".to_string());
}
//➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
//suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
#[test]
fn test_downgradear_usuario_super_a_classic(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "Super".to_string(), "efectivo".to_string());
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1] } ;

    assert_eq!(csr.clientes[0].subscripcion.tipo, TipoSubscripcion::Super);

    csr.downgradear_usuario(&"Ana".to_string());

    assert_eq!(csr.clientes[0].subscripcion.tipo, TipoSubscripcion::Classic);
}
#[test]
fn test_downgradear_usuario_classic_a_basic(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "CLAssic".to_string(), "efectivo".to_string());
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1] } ;

    assert_eq!(csr.clientes[0].subscripcion.tipo, TipoSubscripcion::Classic);

    csr.downgradear_usuario(&"Ana".to_string());

    assert_eq!(csr.clientes[0].subscripcion.tipo, TipoSubscripcion::Basic);
}
#[test]
fn test_downgradear_usuario_basic_a_inactivo(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "Basic".to_string(), "efectivo".to_string());
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1] } ;

    assert_eq!(csr.clientes[0].subscripcion.tipo, TipoSubscripcion::Basic);

    csr.downgradear_usuario(&"Ana".to_string());

    assert_eq!(csr.clientes.last().unwrap().activo, false);
}
//➢ Dado un usuario cancelar la suscripción.
#[test]
fn test_cancelar_subcripcion_usuario(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "Basic".to_string(), "efectivo".to_string());
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1] } ;

    assert_eq!(csr.cancelar_subscripcion(&"Ana".to_string()), true);
    assert_eq!(csr.clientes.last().unwrap().activo, false);
}
#[test]
fn test_cancelar_subcripcion_usuario_no_existe(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "Basic".to_string(), "efectivo".to_string());
    let mut csr:StreamingRust = StreamingRust { clientes: vec![usuario_1] } ;

    assert_eq!(csr.cancelar_subscripcion(&"Ema".to_string()), false);
    assert_eq!(csr.clientes.last().unwrap().activo, true);
}
//➢ Saber el medio de pago que es más utilizado por los usuarios sobre las suscripciones activas
#[test]
fn test_top_active_payment(){
    let usuario_1:Usuario = Usuario::new("Ana".to_string(), "classic".to_string(), "efectivo".to_string());
    let usuario_2:Usuario = Usuario::new("Ema".to_string(), "super".to_string(), "tarjeta de credito".to_string());
    let usuario_3:Usuario = Usuario::new("Oca".to_string(), "basic".to_string(), "efectivo".to_string());
    let usuario_4:Usuario = Usuario::new("Yma".to_string(), "basic".to_string(), "efectivo".to_string());
    let usuario_5:Usuario = Usuario::new("Che".to_string(), "basic".to_string(), "efectivo".to_string());
    let csr:StreamingRust = StreamingRust { clientes: vec![usuario_1, usuario_2, usuario_3, usuario_4, usuario_5] } ;
    
    assert_eq!(csr.top_active_payment(), "Efectivo");

}

