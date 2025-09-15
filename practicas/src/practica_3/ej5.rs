/*  5- Escribir un programa que defina una estructura Producto que tenga campos para el
    nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
    siguientes métodos:
        
        ➢new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
        
        ➢calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
        el precio bruto
        
        ➢aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
        descuento sobre el precio bruto
        
        ➢calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
        precio total a pagar aplicando impuesto y descuento.
        
        Tenga en cuenta que los parámetros son opcionales.
*/

struct Producto {
    nombre: String,
    precio_bruto: f32,
    id_number: u32,
}
impl Producto{
    fn new(nombre:String, precio_bruto:f32, id_number:u32) -> Producto{
        Producto{
            nombre,
            precio_bruto,
            id_number,
        }
    }
    fn aplicar_descuento(&self, &porcentaje_de_descuento:&Option<f32>) -> f32{
        if let Some(descuento) = porcentaje_de_descuento {
            return descuento * self.precio_bruto / 100.00;
        }else{
            return 0.0;
        }
    }
    fn calcular_impuestos(&self, &porcentaje_de_impuestos:&Option<f32>) -> f32{
        if let Some(impuesto) = porcentaje_de_impuestos {
            return impuesto * self.precio_bruto / 100.00;
        }else{
            return 0.0;
        }
    }
    fn calcular_precio_total(&self, &porcentaje_de_impuestos:&Option<f32>, &porcentaje_de_descuento:&Option<f32>) -> f32 {
        return self.precio_bruto - 
        self.aplicar_descuento(&porcentaje_de_descuento) + 
        self.calcular_impuestos(&porcentaje_de_impuestos);
    }

}

pub fn ejercicio_5(){
    let prod:Producto = Producto::new("Producto A".to_string(), 1000.0, 123);
    let impuesto:Option<f32> = Some(10.0); 
    let descuento:Option<f32> = Some(5.0);
    println!("{}, {}, ${}", prod.nombre, prod.id_number, prod.calcular_precio_total(&impuesto,&descuento) );
}

#[test]
fn test_calcular_precio_total(){
    let prod:Producto = Producto::new("Producto A".to_string(), 1000.0, 123);
    assert_eq!(prod.calcular_precio_total(&None, &None), 1000.0);
}
#[test]
fn test_calcular_precio_total_2(){
    let prod:Producto = Producto::new("Producto A".to_string(), 1000.0, 123);
    assert_eq!(prod.calcular_precio_total(&Some(10.0), &None), 1100.0);
}
#[test]
fn test_calcular_precio_total_3(){
    let prod:Producto = Producto::new("Producto A".to_string(), 1000.0, 123);
    assert_eq!(prod.calcular_precio_total(&None, &Some(10.0)), 900.0);
}
#[test]
fn test_calcular_precio_total_4(){
    let prod:Producto = Producto::new("Producto A".to_string(), 1000.0, 123);
    assert_eq!(prod.calcular_precio_total(&Some(10.0), &Some(10.0)), 1000.0);
}