/*
    2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la longitud y el
    ancho. Para dicha estructura implemente los siguientes métodos:
        ➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo retorna.
        ➢ calcular_area: calcular el área y la retorna.
        ➢ calcular_perimetro: calcula el perímetro y lo retorna.
        ➢ es_cuadrado: retorna true si es cuadrado, false caso contrario
*/
#[derive(Debug)]
    pub struct Rectángulo{
        longitud: u16,
        ancho: u16,
    }
    impl Rectángulo {
        fn new(longitud: u16, ancho: u16) -> Rectángulo {
            Rectángulo{
                longitud,
                ancho,
            }
        }
        fn calcular_area(&self) -> u16 {
            return self.ancho*self.longitud;
        }
        fn calcular_perimetro(&self) -> u16 {
            return self.ancho*2 + self.longitud*2;
        }
        fn es_cuadrado(&self) -> bool {
            return self.longitud==self.ancho;
        }
    }

    pub fn ejercicio2(){
        let rectangulito = Rectángulo::new(3, 5);
        println!("{:#?}", rectangulito);
        println!("Es cuadrado? {}", rectangulito.es_cuadrado());
        println!("Perimetro: {}", rectangulito.calcular_perimetro());
        println!("Area: {}", rectangulito.calcular_area());
    }

    //Test ~ Ejercicio 2
    #[test]
    fn test_calcular_area(){
        let rec:Rectángulo = Rectángulo::new(4, 2);
        assert_eq!(rec.calcular_area(), 8);
    }
    #[test]
    fn test_calcular_perimetro(){
        let rec:Rectángulo = Rectángulo::new(4, 2);
        assert_eq!(rec.calcular_perimetro(), 12);
    }
    #[test]
    fn test_es_cuadrado(){
        let rectangulo:Rectángulo = Rectángulo::new(10, 9);
        let cuadrado:Rectángulo = Rectángulo::new(10, 10);
        assert_eq!(rectangulo.es_cuadrado(), false);
        assert_eq!(cuadrado.es_cuadrado(), true);
    }
    
