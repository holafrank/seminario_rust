/*
    4 -Se requiere implementar un sistema de ventas de productos. De cada producto se conoce el nombre,
    una categoría y un precio base, y algunos productos pueden tener descuentos aplicables dependiendo
    de la categoría. Además, se debe registrar al vendedor que realizó la venta y al cliente. De ellos
    se conoce nombre, apellido, dirección, dni y del vendedor nro de legajo, antigüedad y salario. Los
    clientes pueden tener un beneficio de descuento si tienen suscripción al newsletter, de ser así se
    tiene el correo electrónico del mismo.
    El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado. Los
    medios de pago aceptados son: tarjeta de crédito, tarjeta de débito, transferencia bancaria y efectivo.
    Implemente las estructuras, funciones asociadas y traits necesarios para resolver las siguientes acciones:
    ➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de productos.
    ➢ Calcular el precio final de una venta en base a los productos que hay en ella. Para calcularlo tenga
    en cuenta que pueden haber determinados productos de alguna categoría donde debería aplicarse un descuento.
    Tanto la categoría como el porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir
    el sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe aplicar un
    porcentaje de descuento general si el cliente tiene suscripción al newsletter.
    ➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que permita
    visualizar las ventas totales por categoría de producto y otro por vendedor.


    ATENCIÓN
    Esta es una posible solución al ejercicio 4 de una manera "más objetosa" usando Box para lograr polimorfismo
    entre los métodos concretos de las distintas categorías (¡Patrón Strategy!). Con Box podemos hacer que un
    Vector de Productos contenga Productos de distintas Categorías, por ejemplo:
    [Producto<Nuevo>, Producto<Oferta>, Producto<Nuevo>, Producto<Nuevo>, Producto<Estandar>].

    Si quisieramos agregar una nueva categoría, sólo tendríamos que:
    **Hacer un nuevo struct**
    #[derive(Debug, PartialEq)]
    struct NuevaCategoría {
        descripcion: String
    }
    **Una descripción... o no**
    impl NuevaCategoría {
        fn new() -> NuevaCategoría {
            NuevaCategoría { descripcion : "Producto con una categoría completamente nueva.".to_string() }
        }
    }
    impl Categoria for NuevaCategoría{
        fn descuento_categoria(&self) -> f64{
            75.0    // ¡¡Qué barato!!
        }
    }

    Y listo, ya Producto sabrá dónde buscar la implementación de descuento_categoria(),
    y puede ser agregado al Vector de Productos sin problema.

*/

use core::panic;
use chrono::{DateTime, Datelike, Local, TimeZone, Weekday};

struct Producto {
    nombre : String,
    categoria: Box<dyn Categoria>, // Usamos un **trait object** para que los productos puedan ser agregados a un mismo vector aunque tengan categorías diferentes.
    precio_base : f64,
    descuento_newsletter : f64
}
impl Producto {
    pub fn new(nombre:String, categoria: Box<dyn Categoria>, precio_base:f64) -> Producto {
        Producto { nombre, categoria, precio_base, descuento_newsletter : 10.0 }
    }
         
    pub fn get_precio_total(&self, aplica:bool) -> f64{
        let precio = self.porcentaje(self.categoria.descuento_categoria(), self.precio_base);
        if aplica {
            return self.porcentaje(self.descuento_newsletter, precio);
        } else {
            return precio
        }
    }
    pub fn porcentaje(&self, porcentaje_descuento:f64, base:f64) -> f64 {
        let aux = porcentaje_descuento * base / 100.0;
        return  base - aux;
    }
}

/*
    Un **trait object** sirve para que los productos puedan ser agregados a un mismo vector aunque tengan categorías diferentes.
    ¡No podemos saber en tiempo de compilación cuánto puede llegar a ocupar un tipo concreto de Categoría
    porque no sabemos qué tipo concreto va a ser hasta que ejecutemos!
    Usar un **trait object** es una forma de almacenar cualquier tipo que implemente el trait Categoria en un contenedor de tamaño fijo.
    (en este caso, un Box).
    ¿Qué es ese "dyn"?
    dyn es una abreviatura de "dynamic".
    Se usa para indicar que el tipo concreto del trait no se conoce en tiempo de compilación,
    sino que se resuelve en tiempo de ejecución.
    Es como decirle al compilador: "Este es un objeto que implementa el trait Categoria, pero no sé exactamente qué tipo concreto es".
*/

pub trait Categoria{
    fn descuento_categoria(&self) -> f64;
}
#[derive(Debug, PartialEq)]
struct Nuevo {
    descripcion: String
}
impl Nuevo {
    fn new() -> Nuevo {
        Nuevo { descripcion: "Producto novedoso.".to_string() }
    }
}
impl Categoria for Nuevo{
    fn descuento_categoria(&self) -> f64{
        0.0
    }
}
#[derive(Debug, PartialEq)]
struct Oferta {
    descripcion: String
}
impl Oferta {
    fn new() -> Oferta {
        Oferta { descripcion : "Producto con precio reducido.".to_string() }
    }
}
impl Categoria for Oferta{
    fn descuento_categoria(&self) -> f64{
       10.0
    }
}
#[derive(Debug, PartialEq)]
struct Especial {
    descripcion: String
}
impl Especial {
    fn new() -> Especial {
        Especial { descripcion: "Producto de edición limitada o especial de temporada.".to_string() }
    }
}
impl Categoria for Especial{
    fn descuento_categoria(&self) -> f64{
       5.0
    }
}
#[derive(Debug, PartialEq)]
struct Estandar {
    descripcion: String
}
impl Estandar {
    fn new() -> Estandar {
        Estandar { descripcion: "Producto estándar. Lo podemos encontrar todos los días en cualquier góndola. Tiene descuento todos los jueves.".to_string() }
    }
}
impl Categoria for Estandar{
    fn descuento_categoria(&self) -> f64{
        if Local::now().weekday() == chrono::Weekday::Thu {
            25.0
        } else {
            0.0
        }
    }
}

//

struct Venta {
    fecha: DateTime<Local>,
    cliente: Cliente,
    vendedor: Vendedor,
    productos: Vec<Producto>, // ¡Ahora sí el vector puede contener cualquier Producto de cualquier Categoria!
    medio_de_pago: MedioDePago
}
impl Venta {
    pub fn new(cliente: Cliente, vendedor:Vendedor, productos: Vec<Producto>, medio_de_pago: MedioDePago) -> Venta {
        Venta { fecha: Local::now(), cliente, vendedor, productos, medio_de_pago }
    }
    pub fn precio_total(&self) -> f64 {
        let mut total: f64 = 0.0;
        for p in &self.productos {
            total += p.get_precio_total(self.cliente.has_newsletter());
        }
        total
    }
}

/*
    Ahora el vector puede contener cualquier Producto de cualquier Categoria, siendo las Categorías de Tipos de Struct distintos.
*/

#[derive(Debug, PartialEq)]
struct Vendedor {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
    legajo: u16,
    antigüedad: u8,
    salario: f64
}
impl Vendedor {
    pub fn new(nombre: String, apellido: String, direccion: String, dni: u32, legajo: u16, antigüedad: u8, salario: f64 ) -> Vendedor {
        Vendedor { nombre, apellido, direccion, dni, legajo, antigüedad, salario }
    }
}

//

#[derive(Debug, PartialEq)]
struct Cliente {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
    email: Option<String>
}
impl Cliente {
    pub fn new(nombre: String, apellido: String, direccion: String, dni: u32, email: Option<String> ) -> Cliente {
        Cliente { nombre, apellido, direccion, dni, email }
    }
    pub fn subscribe_newsletter(&mut self, email:String)  {
        self.email = Some(email);
    }
    pub fn unsubscribe_newsletter(&mut self) {
        self.email = None;
        
    }
    pub fn has_newsletter(&self) -> bool {
        if self.email == None {
            return false;
        } else {
            return true;
        }
    }
}

//

enum MedioDePago {
    TarjetaDeCredito,
    TarjetaDeDebito,
    Transferencia,
    Efectivo
}
impl MedioDePago {
    pub fn new(medio_de_pago:String) -> MedioDePago {
        let m = medio_de_pago.to_lowercase();
        match m.as_str() {
            "tarjeta de credito" => MedioDePago::TarjetaDeCredito,
            "tarjeta de debito" => MedioDePago::TarjetaDeDebito,
            "tarjeta de crédito" => MedioDePago::TarjetaDeCredito,
            "tarjeta de débito" => MedioDePago::TarjetaDeDebito,
            "transferencia" => MedioDePago::Transferencia,
            "efectivo" => MedioDePago::Efectivo,
            _ => panic!("¡Medio de pago no reconocido!"),
        }
    }
}


pub fn ejercicio4() {

    let cliente = Cliente::new(
        "Juan Carlos".to_string(),
        "Bodoque".to_string(),
        "Calle Falsa 123".to_string(),
        12345678,
        Some("juanca@mail.com".to_string()),
    );

    let vendedor = Vendedor::new(
        "Rosario".to_string(),
        "Central".to_string(),
        "Ministro Carbajal y Ministro Carbajal".to_string(),
        87654321,
        123,
        5,
        50000.0,
    );

    let producto1 = Producto::new("Pelota de Fútbol".to_string(), Box::new(Nuevo::new()), 10_000.0);
    let producto2 = Producto::new("Torta de Ricota".to_string(), Box::new(Oferta::new()), 500.0);

    let venta = Venta::new(
        cliente,
        vendedor,
        vec![producto1, producto2],
        MedioDePago::Efectivo,
    );

    println!("Precio total de la venta: {}", venta.precio_total());

}

#[test]
fn test_precio_total_newsletter_mismo_tipo(){
    let cliente = Cliente::new("Juan Carlos".to_string(), "Bodoque".to_string(), "Calle Falsa 123".to_string(),12345678, Some("juanca@mail.com".to_string()));
    let vendedor = Vendedor::new("Rosario".to_string(), "Central".to_string(), "Ministro Carbajal y Ministro Carbajal".to_string(), 87654321, 123, 5, 50000.0);

    let producto1 = Producto::new("Pelota de Fútbol".to_string(), Box::new(Nuevo::new()), 10_000.0);
    let producto2 = Producto::new("Botines Azules".to_string(), Box::new(Nuevo::new()), 5_000.0);

    let venta = Venta::new(cliente, vendedor, vec![producto1, producto2], MedioDePago::Efectivo);

    // descuento_categoria = 0%
    // descuento_newsletter = 10%
    // total = 15_000 - 10% =  

    assert_eq!(venta.precio_total(), 13_500.0);

}

#[test]
fn test_precio_total_newsletter_distinto_tipo(){
    let cliente = Cliente::new("Juan Carlos".to_string(), "Bodoque".to_string(), "Calle Falsa 123".to_string(),12345678, Some("juanca@mail.com".to_string()));
    let vendedor = Vendedor::new("Rosario".to_string(), "Central".to_string(), "Ministro Carbajal y Ministro Carbajal".to_string(), 87654321, 123, 5, 50000.0);

    let producto1 = Producto::new("Pelota de Fútbol".to_string(), Box::new(Nuevo::new()), 10_000.0);
    let producto2 = Producto::new("Botines Azules".to_string(), Box::new(Oferta::new()), 10_000.0);
    let producto4 = Producto::new("Avellanas Bañadas en Chocolate".to_string(), Box::new(Especial::new()), 10_000.0);

    let venta = Venta::new(cliente, vendedor, vec![producto1, producto2, producto4], MedioDePago::Efectivo);

    // descuento_categoria_nuevo = 10_000 - 0% = 10_000
    // descuento_categoria_oferta = 10_000 - 10% = 9_000
    // descuento_categoria_especial = 10_000 - 5% = 9_500
    // descuento_newsletter = 10%
    // total = 30_000 =>  25_650

    assert_eq!(venta.precio_total(), 25_650.0);

}

#[test]
fn test_precio_total_mismo_tipo(){
    let cliente = Cliente::new("Juan Carlos".to_string(), "Bodoque".to_string(), "Calle Falsa 123".to_string(),12345678, None);
    let vendedor = Vendedor::new("Rosario".to_string(), "Central".to_string(), "Ministro Carbajal y Ministro Carbajal".to_string(), 87654321, 123, 5, 50000.0);

    let producto1 = Producto::new("Pelota de Fútbol".to_string(), Box::new(Nuevo::new()), 10_000.0);
    let producto2 = Producto::new("Lápices Acuarelables".to_string(), Box::new(Nuevo::new()), 10_000.0);

    let venta = Venta::new(cliente, vendedor, vec![producto1, producto2], MedioDePago::Efectivo);

    // descuento_categoria_nuevo = 10_000 - 0% = 10_000
    // descuento_categoria_nuevo = 10_000 - 0% = 10_000
    // descuento_newsletter = 0%
    // total = 20_000 =>  20_000

    assert_eq!(venta.precio_total(), 20_000.0);

}

#[test]
fn test_precio_total_distinto_tipo(){
    let cliente = Cliente::new("Juan Carlos".to_string(), "Bodoque".to_string(), "Calle Falsa 123".to_string(),12345678, None);
    let vendedor = Vendedor::new("Rosario".to_string(), "Central".to_string(), "Ministro Carbajal y Ministro Carbajal".to_string(), 87654321, 123, 5, 50000.0);

    let producto1 = Producto::new("Pelota de Fútbol".to_string(), Box::new(Nuevo::new()), 10_000.0);
    let producto2 = Producto::new("Botines Azules".to_string(), Box::new(Oferta::new()), 10_000.0);
    let producto4 = Producto::new("Avellanas Bañadas en Chocolate".to_string(), Box::new(Especial::new()), 10_000.0);

    let venta = Venta::new(cliente, vendedor, vec![producto1, producto2, producto4], MedioDePago::Efectivo);

    // descuento_categoria_nuevo = 10_000 - 0% = 10_000
    // descuento_categoria_oferta = 10_000 - 10% = 9_000
    // descuento_categoria_especial = 10_000 - 5% = 9_500
    // descuento_newsletter = 0%
    // total = 30_000 =>  28_500

    assert_eq!(venta.precio_total(), 28_500.0);

}

#[test]
fn test_feliz_jueves(){
    let cliente = Cliente::new("Juan Carlos".to_string(), "Bodoque".to_string(), "Calle Falsa 123".to_string(),12345678, None);
    let vendedor = Vendedor::new("Rosario".to_string(), "Central".to_string(), "Ministro Carbajal y Ministro Carbajal".to_string(), 87654321, 123, 5, 50000.0);

    let producto1 = Producto::new("Caballo de Cristal".to_string(), Box::new(Estandar::new()), 10_000.0);
    let producto2 = Producto::new("Buda de Ojos Azules".to_string(), Box::new(Nuevo::new()), 10_000.0);
    let producto4 = Producto::new("Botellas Sin Fondo".to_string(), Box::new(Estandar::new()), 10_000.0);

    let venta = Venta::new(cliente, vendedor, vec![producto1, producto2, producto4], MedioDePago::Efectivo);

    // descuento_categoria_nuevo = 10_000 - 0% = 10_000
    // descuento_categoria_estandar = 10_000 - 25% = 7_500
    // descuento_categoria_estandar = 10_000 - 25% = 7_500
    // descuento_newsletter = 0%
    // total = 30_000 =>  25_000

    if Local::now().weekday() == chrono::Weekday::Thu {
        assert_eq!(venta.precio_total(), 25_000.0);
    } else {
        assert_eq!(venta.precio_total(), 30_000.0);
    }
    
}