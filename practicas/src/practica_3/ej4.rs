/*
    4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
    longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
        ➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
        ➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero,
        isósceles o escaleno.
        ➢ calcular_area: calcular el área y la retorna.
        ➢ calcular_perimetro: calcula el perímetro y lo retorna. 
*/
enum Tipo{
    Isosceles,
    Escaleno,
    Equilatero,
}
pub struct Triangulo{
    lado_1: f64,
    lado_2: f64,
    lado_3: f64,
    tipo: Tipo,
}
impl Triangulo {
    fn new(lado_1:f64, lado_2:f64, lado_3:f64) -> Triangulo {
        if lado_1 != lado_2 && lado_1!=lado_3 && lado_2 != lado_3{
            let tipo:Tipo = Tipo::Escaleno;
            Triangulo{
                lado_1,
                lado_2,
                lado_3,
                tipo,
            }
        }else{
            if lado_1 == lado_2 && lado_1== lado_3 && lado_2 == lado_3{
                let tipo:Tipo = Tipo::Equilatero;
                Triangulo{
                    lado_1,
                    lado_2,
                    lado_3,
                    tipo,
                }
            }else{
                let tipo:Tipo = Tipo::Isosceles;
                Triangulo{
                    lado_1,
                    lado_2,
                    lado_3,
                    tipo,
                }
            }
       }
    }

    fn determinar_tipo(&self) -> Tipo {
        match self.tipo{
            Tipo::Isosceles => Tipo::Isosceles,
            Tipo::Equilatero => Tipo::Equilatero,
            Tipo::Escaleno => Tipo::Escaleno,
        }
    }
    fn area_isosceles(&self, &base:&f64, &lados:&f64) -> f64{
        // area = base * altura / 2 
        // altura = [lado² - (base²/4)]^0.5
        let altura:f64= (lados.powf(2.00) - (base.powf(2.00) / 4.00)).sqrt();
        return base * altura/2.00;
    }
    fn area_equilatero(&self, &lados:&f64) -> f64{
        // area = [ (3^0.5) / 4 ] * lados²
        let area:f64= 3.00;
        return (area.sqrt()/4.00)* lados.powf(2.00);
    }
    fn base_isosceles(&self) -> f64{
        if self.lado_1 != self.lado_2 && self.lado_1 != self.lado_3{
            return self.lado_1;
        }else{
            if self.lado_2 != self.lado_1 && self.lado_2 != self.lado_3{
                    return self.lado_2;
                }else{
                    return self.lado_3;
                }
            }    
    }
    fn lados_isosceles(&self) -> f64 {
        if self.lado_1 == self.lado_2{
            return self.lado_1;
        }else{
            if self.lado_2 == self.lado_3{
                return self.lado_2;
            }else{
                return self.lado_3;
            }
        }
    }
    fn semiperimetro(&self) -> f64{
        return self.calcular_perimetro() / 2.00;
    }
    fn area_escaleno(&self) -> f64 {
        // semiperimetro = perimetro / 2
        // area = [S* (S-a)*(S-b)*(S-a)*]
        return (self.semiperimetro()*
        (self.semiperimetro() - self.lado_1)*
        (self.semiperimetro() - self.lado_2)*
        (self.semiperimetro() - self.lado_3)).sqrt();
    }
    fn calcular_area(&self) -> f64 {
        if self.lado_cero(){ return 0.0;
        }else{
            match self.tipo{
                Tipo::Isosceles => return self.area_isosceles(&self.base_isosceles(), &self.lados_isosceles()),
                Tipo::Equilatero => return self.area_equilatero(&self.lado_1),
                Tipo::Escaleno => return self.area_escaleno(),
            }
        }
    }
    fn calcular_perimetro(&self) -> f64 {
        return self.lado_1 + self.lado_2 + self.lado_3;
    }
    fn lado_cero(&self) -> bool {
        return (self.lado_1 == 0.0) || (self.lado_2 == 0.0) || (self.lado_3 == 0.0);
    }
}

pub fn ejercicio4(){

    let tri_escaleno:Triangulo = Triangulo::new(4.00,5.00,3.00); 
    println!("Area: {:?} Perimetro: {:?}", tri_escaleno.calcular_area(), tri_escaleno.calcular_perimetro());
    match tri_escaleno.determinar_tipo(){
        Tipo::Equilatero => println!("Es Equilatero"),
        Tipo::Isosceles => println!("Es Isosceles"),
        Tipo::Escaleno => println!("Es Escaleno"),
    }    
        
    println!("---");
    let tri_equilatero:Triangulo = Triangulo::new(4.00,4.00,4.00); 
    println!("Area: {:?} Perimetro: {:?}", tri_equilatero.calcular_area(), tri_equilatero.calcular_perimetro());
    match tri_equilatero.determinar_tipo(){
        Tipo::Equilatero => println!("Es Equilatero"),
        Tipo::Isosceles => println!("Es Isosceles"),
        Tipo::Escaleno => println!("Es Escaleno"),
    }
        
    println!("---");
    let tri_isosceles:Triangulo = Triangulo::new(4.00,4.00,6.00); 
    println!("Area: {:?} Perimetro: {:?}", tri_isosceles.calcular_area(), tri_isosceles.calcular_perimetro());
    match tri_isosceles.determinar_tipo(){
        Tipo::Equilatero => println!("Es Equilatero"),
        Tipo::Isosceles => println!("Es Isosceles"),
        Tipo::Escaleno => println!("Es Escaleno"),
    }        
}


#[test]
fn test_perimetro_escaleno(){
    let tri_escaleno:Triangulo = Triangulo::new(1.00,2.00,3.00);
    assert_eq!(tri_escaleno.calcular_perimetro(), 6.0);
}
#[test]
fn test_perimetro_equilatero(){
    let tri_equilatero:Triangulo = Triangulo::new(1.00,1.00,1.00);
    assert_eq!(tri_equilatero.calcular_perimetro(), 3.0);
}
#[test]
fn test_perimetro_isosceles(){
    let tri_isosceles:Triangulo = Triangulo::new(1.00,2.00,2.00);
    assert_eq!(tri_isosceles.calcular_perimetro(), 5.0);
}
#[test]
fn test_perimetro_cero(){
    let tri_cero:Triangulo = Triangulo::new(0.00,0.00,0.00);
    assert_eq!(tri_cero.calcular_perimetro(), 0.0);
}
#[test]
fn test_area_escaleno(){
    let tri_escaleno:Triangulo = Triangulo::new(4.00,5.00,3.00);
    assert_eq!(tri_escaleno.calcular_area(), 6.0);
}
#[test]
fn test_area_equilatero(){
    let tri_equilatero:Triangulo = Triangulo::new(4.00,4.00,4.00);
    assert_eq!(tri_equilatero.calcular_area(), 6.928203230275509);
}
#[test]
fn test_area_isosceles(){
    let tri_isosceles:Triangulo = Triangulo::new(4.00,4.00,6.00); 
    assert_eq!(tri_isosceles.calcular_area(), 7.937253933193772);
}
#[test]
fn test_area_cero(){
    let tri_cero:Triangulo = Triangulo::new(1.00,2.00,0.00);
    assert_eq!(tri_cero.calcular_area(), 0.0);
}
