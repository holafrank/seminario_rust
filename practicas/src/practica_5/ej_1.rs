/* 
    1- En base al ejercicio 7 del tp#3 implemente lo siguiente:
        a- Al agregar un auto si supera el límite de la concesionaria debe arrojar un error
        propio con un mensaje de contexto.
        b- Haga todos los tests correspondientes para probar en profundidad los métodos
        que agregan un auto y eliminan un auto de la concesionaria, obteniendo el mayor
        porcentaje de coverage sobre el código que realiza las operaciones.
        c- Una vez hecho el punto anterior debe hacer que los autos de la concesionaria se
        almacenen en un archivo en formato JSON. Agregue y modifique lo que considere
        necesario para que:
            - Al agregar un nuevo auto se abra el archivo de autos guardados y lo agregue a
            dicho archivo, para ello debería abrirlo en modo r+w mire lo siguiente para ver cómo
            realizarlo: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
            - Eliminar un auto: al eliminar un auto se debe eliminar este del archivo.
            No debe modificar los tests hechos en el punto b. Si puede agregar más en caso de que
            haga nueva funcionalidad..
*/

use std::fs::File;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;

struct ConcesionarioAuto{
    nombre: String,
    dirección: String,
    capacidad:u16,
    lista_de_autos: Vec<Auto>,
}
impl ConcesionarioAuto {
    pub fn new(nombre: String, dirección: String, capacidad:u16,) -> ConcesionarioAuto {
        
        let path = "src/listado_de_autos.txt";
        let mut archivo = match File::create(path) {
            Err(e) => panic!("No se pudo crear. \nError: {}", e),
            Ok(archivo) => archivo,
        };

        ConcesionarioAuto { nombre, dirección, capacidad,
            lista_de_autos:Vec::with_capacity(capacidad as usize),
        }

    }
    /// Al agregar un auto si supera el límite de la concesionaria debe arrojar un error propio con un mensaje de contexto.
    pub fn vector_lleno(&self) -> bool {
        if self.lista_de_autos.len() as u16 == self.capacidad {
            panic!("No hay más lugar para un nuevo auto =(");
        }else{
            return false;
        }
    }
    /// Al agregar un nuevo auto se abra el archivo de autos guardados y lo agregue adicho archivo
    pub fn agregar_auto(&mut self, auto:Auto) -> bool{
        if !self.vector_lleno(){

            let path = "src/listado_de_autos.txt";
            let mut archivo = match OpenOptions::new().append(true).open(path) {
                Err(e) => panic!("No se pudo escribir el archivo. \nError:", e),
                Ok(mut archivo) => {
                    let auto_json = match serde_json::to_string_pretty(&auto){
                        Err(e) => panic!("No se pudo serializar. \nError:: {}", e),
                        Ok(j) => { archivo.write(&j.as_bytes() );
                        },
                    };
                },
            };

            self.lista_de_autos.push(auto);

        }
        return true;
    }
    pub fn buscar_auto(&self, auto:&Auto) -> Option<&Auto>{
        for i in 0..self.lista_de_autos.len(){
            if self.lista_de_autos[i].año == auto.año &&
            self.lista_de_autos[i].marca == auto.marca &&
            self.lista_de_autos[i].modelo == auto.modelo &&
            self.lista_de_autos[i].precio_bruto == auto.precio_bruto &&
            self.lista_de_autos[i].color == auto.color {
                return Some(&self.lista_de_autos[i]);
            }
        }
        return None;
    }
    pub fn buscar_auto_dar_indice(&self, auto:Auto) -> Option<usize>{
        for i in 0..self.lista_de_autos.len() - 1{
            if self.lista_de_autos[i].año == auto.año &&
            self.lista_de_autos[i].marca == auto.marca &&
            self.lista_de_autos[i].modelo == auto.modelo &&
            self.lista_de_autos[i].precio_bruto == auto.precio_bruto &&
            self.lista_de_autos[i].color == auto.color {
                return Some(i);
            }
        }
        return None;
    }
    pub fn eliminar_auto(&mut self, auto:Auto) -> bool {
        match self.buscar_auto_dar_indice(auto) {
                Some(indice) => {
                    self.lista_de_autos.remove(indice);
                    return true;
                },
                None => return false,
            }
        }    
}

#[derive(Debug, PartialEq, Eq)]
enum Color{
    Rojo,
    Azul,
    Amarillo,
    Verde,
    Blanco,
    Negro
}
impl Color {
    pub fn new(color_string:String) -> Color {
        let c:String = color_string.to_lowercase();
        match c.as_str() {
            "blanco" => Color::Blanco,
            "negro" => Color::Negro,
            "rojo" =>  Color::Rojo,
            "azul" =>  Color::Azul,
            "amarillo" => Color::Amarillo,
            "verde" =>  Color::Verde,
            _ => panic!("Este color no está disponible!")
        }
    }
    pub fn mi_color(&self) -> Color {
        match self {
            Color::Rojo => return Color::Rojo,
            Color::Amarillo => return Color::Amarillo,
            Color::Azul => return Color::Azul,
            Color::Blanco => return Color::Blanco,
            Color::Negro => return Color::Negro,
            Color::Verde => return Color::Verde
        }
    } 
}

#[derive(Debug)]
struct Auto{
    marca:String,
    modelo:String,
    año:u16,
    color: Color,
    precio_bruto:f64,
}
impl Auto{
    pub fn new(marca:String, modelo:String, año:u16, color_string:String, precio_bruto:f64) -> Auto {
        let color:Color = Color::new(color_string);
        Auto {marca, modelo, año, color, precio_bruto}
    }
    pub fn recargo_bmw(&self) -> f64 {
        if self.marca == "BMW".to_string(){
            return (self.precio_bruto * (15.0)) / 100.0;
        }else{
            return 0.0;
        }
    }
    pub fn descuento_antiguedad(&self) -> f64 {
        if self.año < 2000 {
            return (self.precio_bruto * (-5.0 )) / 100.0;
        }else{
            return 0.0;
        }
    }
    pub fn modificador_color(&self) -> f64 {
        if  (self.color == Color::Rojo) || (self.color == Color::Amarillo) || (self.color == Color::Azul) {
            return (self.precio_bruto * 25.0) / 100.0
        }else{
            return (self.precio_bruto * (-10.0) ) / 100.0
        }
    }
    pub fn calcular_precio(&self) -> f64 {        
        return self.precio_bruto + self.recargo_bmw() + self.descuento_antiguedad() + self.modificador_color();
    }
}

#[test]
fn test_agregar_auto(){
    let mut consesionaria:ConcesionarioAuto =
        ConcesionarioAuto::new("Nombre".to_string(), "Dirección".to_string(), 3);

    let auto_ejemplo:Auto = 
        Auto::new("Ford".to_string(), "K".to_string(), 2006, "AZUL".to_string(), 1_000_000.0);

    assert_eq!(consesionaria.agregar_auto(auto_ejemplo), true);
    assert_eq!(consesionaria.lista_de_autos[0].marca,"Ford".to_string());
    assert_eq!(consesionaria.lista_de_autos[0].modelo,"K".to_string());
    assert_eq!(consesionaria.lista_de_autos[0].año,2006);
    assert_eq!(consesionaria.lista_de_autos[0].color,Color::Azul);
    assert_eq!(consesionaria.lista_de_autos[0].precio_bruto, 1_000_000.0);
    assert_eq!(consesionaria.lista_de_autos.len(), 1);
}
#[test]
#[should_panic(expected = "No hay más lugar para un nuevo auto =(")]
fn test_agregar_auto_fallo(){
    let mut consesionaria:ConcesionarioAuto = 
        ConcesionarioAuto::new("Nombre".to_string(), "Dirección".to_string(), 1);

    let auto_no_entra:Auto = 
        Auto::new("Fiat".to_string(), "600".to_string(), 1996, "ROJO".to_string(), 2_000_000.0);
    let auto_entra:Auto = 
        Auto::new("Ford".to_string(), "K".to_string(), 2006, "AZUL".to_string(), 1_000_000.0);

    consesionaria.agregar_auto(auto_entra);
    consesionaria.agregar_auto(auto_no_entra);
}
#[test]
fn test_eliminar_auto(){
    let mut consesionaria:ConcesionarioAuto = 
        ConcesionarioAuto::new("Nombre".to_string(), "Dirección".to_string(), 3);

    let auto_1:Auto = 
        Auto::new("Fiat".to_string(), "600".to_string(), 1996, "ROJO".to_string(), 2_000_000.0);
    let auto_2:Auto = 
        Auto::new("Ford".to_string(), "K".to_string(), 2006, "AZUL".to_string(), 1_000_000.0);
    
    consesionaria.agregar_auto(auto_1);
    consesionaria.agregar_auto(auto_2);
    
    let auto_a_borrar:Auto = 
        Auto::new("Fiat".to_string(), "600".to_string(), 1996, "ROJO".to_string(), 2_000_000.0);
    
    assert_eq!(consesionaria.eliminar_auto(auto_a_borrar), true);
    assert_eq!(consesionaria.lista_de_autos.len(), 1);
}
#[test]
fn test_eliminar_auto_fallo(){
    let mut consesionaria:ConcesionarioAuto =
        ConcesionarioAuto::new("Nombre".to_string(), "Dirección".to_string(), 3);
    
    let auto_1:Auto = 
        Auto::new("Fiat".to_string(), "600".to_string(), 1996, "ROJO".to_string(), 2_000_000.0);
    let auto_2:Auto = 
        Auto::new("Ford".to_string(), "K".to_string(), 2006, "AZUL".to_string(), 1_000_000.0);
    
    consesionaria.agregar_auto(auto_1);
    consesionaria.agregar_auto(auto_2);
    
    let auto_a_borrar:Auto = 
        Auto::new("Ford".to_string(), "F100".to_string(), 1968, "ROJO".to_string(), 2_000_000.0);
    
    assert_eq!(consesionaria.eliminar_auto(auto_a_borrar), false);
    assert_eq!(consesionaria.lista_de_autos.len(), 2);
}
#[test]
fn test_buscar_auto(){
    let mut consesionaria:ConcesionarioAuto = 
        ConcesionarioAuto::new("Nombre".to_string(), "Dirección".to_string(), 3);
    
    let auto_1:Auto = 
        Auto::new("Fiat".to_string(), "600".to_string(), 1996, "ROJO".to_string(), 2_000_000.0);
    let auto_2:Auto = 
        Auto::new("Ford".to_string(), "K".to_string(), 2006, "AZUL".to_string(), 1_000_000.0);
    
    consesionaria.agregar_auto(auto_1);
    consesionaria.agregar_auto(auto_2);
    
    let auto_a_buscar:Auto = 
        Auto::new("Ford".to_string(), "K".to_string(), 2006, "AZUL".to_string(), 1_000_000.0);
    
    assert_eq!(consesionaria.buscar_auto(&auto_a_buscar).unwrap().marca, "Ford".to_string());
    assert_eq!(consesionaria.buscar_auto(&auto_a_buscar).unwrap().modelo, "K".to_string());
    assert_eq!(consesionaria.buscar_auto(&auto_a_buscar).unwrap().año, 2006);
    assert_eq!(consesionaria.buscar_auto(&auto_a_buscar).unwrap().color, Color::Azul);
    assert_eq!(consesionaria.buscar_auto(&auto_a_buscar).unwrap().precio_bruto, 1_000_000.0);
}
#[test]
fn test_buscar_auto_fallo(){
    let mut consesionaria:ConcesionarioAuto = 
        ConcesionarioAuto::new("Nombre".to_string(), "Dirección".to_string(), 3);
    let auto_1:Auto = 
        Auto::new("Fiat".to_string(), "600".to_string(), 1996, "ROJO".to_string(), 2_000_000.0);
    let auto_2:Auto = 
        Auto::new("Ford".to_string(), "K".to_string(), 2006, "AZUL".to_string(), 1_000_000.0);
    
    consesionaria.agregar_auto(auto_1);
    consesionaria.agregar_auto(auto_2);
    
    let auto_a_buscar:Auto = 
        Auto::new("Ford".to_string(), "F100".to_string(), 1968, "ROJO".to_string(), 2_000_000.0);
    
    assert_eq!(consesionaria.buscar_auto(&auto_a_buscar).is_none(), true);
}
/*

Casos de prueba:

1) Color primario (+25%), NO es BMW (+0), año mayor o igual a 2000 (+0)
2) Color NO primario (-10%), BMW (+15%), año mayor o igual a 2000 (+0)
3) Color primario (+25%), NO es BMW (+0), año menor a 2000 (-5%)

    ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
    ■ si es de color primario le aplica un recargo del 25%, sino le aplica un descuento del 10%.
    ■ si la marca es BMW le aplica un recargo del 15%
    ■ si el año es menor a 2000 le aplica un descuento del 5%.

*/
#[test]
fn test_calcular_precio_caso_1(){
    let auto_1:Auto = 
        Auto::new("Fiat".to_string(), "600".to_string(), 2000, "ROJO".to_string(), 1_000_000.0);
    
    assert_eq!(auto_1.calcular_precio(), 1_250_000.0);
    assert_eq!(auto_1.modificador_color(), 250_000.0);
    assert_eq!(auto_1.descuento_antiguedad(), 0.0);
    assert_eq!(auto_1.recargo_bmw(), 0.0);
}
#[test]
fn test_calcular_precio_caso_2(){
    let auto_1:Auto = 
        Auto::new("BMW".to_string(), "M240i".to_string(), 2000, "VERDE".to_string(), 1_000_000.0);
    
    assert_eq!(auto_1.calcular_precio(), 1_050_000.0);
    assert_eq!(auto_1.modificador_color(), -100_000.0);
    assert_eq!(auto_1.descuento_antiguedad(), 0.0);
    assert_eq!(auto_1.recargo_bmw(), 150_000.0);
}
#[test]
fn test_calcular_precio_caso_3(){
    let auto_1:Auto = 
        Auto::new("Fiat".to_string(), "600".to_string(), 1999, "AMARILLO".to_string(), 1_000_000.0);
    
    assert_eq!(auto_1.calcular_precio(), 1_200_000.0);
    assert_eq!(auto_1.modificador_color(), 250_000.0);
    assert_eq!(auto_1.descuento_antiguedad(), -50_000.0);
    assert_eq!(auto_1.recargo_bmw(), 0.0);
}