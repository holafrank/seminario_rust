/*
    8- Defina la estructura Cancion con campos para el título, el artista y el género.
    El género puede ser rock, pop, rap, jazz, otros. Luego modele una playlist.
    La playlist está compuesta por una lista de canciones y un nombre,
    y se permiten hacer las siguientes acciones sobre ella:
        ➔ agregar canción. (*)
        ➔ eliminar canción. (*)
        ➔ mover canción // mueve la canción a una determinada posición de la playlist. (*)
        ➔ buscar canción por nombre. (*)
        ➔ obtener las canciones de un determinado género. (*)
        ➔ obtener las canciones de un determinado artista. (*)
        ➔ modificar título de la playlist. (*)
        ➔ eliminar todas las canciones. (*)
*/

#[derive(Debug)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

impl Cancion {

    /// Crea una nueva canción.
    pub fn new (titulo:String, artista:String, genero_string:String) -> Cancion {
        let genero:Genero = Genero::new(genero_string);
        Cancion { titulo, artista, genero }
    }     

    /// Como en esta práctica no podemos usar Traits (salvo Debug), hacemos el Eq a mano.
    pub fn misma_cancion (&self, cancion:&Cancion) -> bool {
        return self.artista == cancion.artista
        && self.titulo == cancion.titulo
        && self.genero.mismo_genero(&cancion.genero);
    }

    /// Devuelve True si el nombre enviado coincide con el título de la canción.
    pub fn mismo_titulo(&self, nombre:&String) -> bool {
        return self.titulo.to_lowercase() == *nombre.to_lowercase();
    }

    /// Devuelve True si el nombre enviado coincide con el artista de la canción.
    pub fn mismo_artista(&self, nombre:&String) -> bool {
        return self.artista.to_lowercase() == *nombre.to_lowercase();
    }

    /// Como en esta práctica no podemos usar Traits (salvo Debug), hacemos el Copy a mano.
    pub fn copiar(&self) -> Cancion {
        let t:String = self.titulo.clone();
        let a:String = self.artista.clone();
        match self.genero{
            Genero::Jazz => return Cancion { titulo: t, artista: a, genero: Genero::Jazz },
            Genero::Otro=> return Cancion { titulo: t, artista: a, genero: Genero::Otro },
            Genero::Pop=> return Cancion { titulo: t, artista: a, genero: Genero::Pop },
            Genero::Rock=> return Cancion { titulo: t, artista: a, genero: Genero::Rock },
            Genero::Rap=> return Cancion { titulo: t, artista: a, genero: Genero::Rap },
        }
    }

}

#[derive(Debug)]
enum Genero{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otro,
}

impl Genero{

    /// Crea una nueva instancia del enum Genero a partir de un string.
    pub fn new(genero_string:String) -> Genero {
        match genero_string.to_lowercase().as_str() {
            "jazz" => return Genero::Jazz,
            "pop" => return Genero::Pop,
            "rock" => return Genero::Rock,
            "rap" => return Genero::Rap,
            "otro" => return Genero::Otro,
            _ => panic!("¡El género ingresado no es válido! \n Géneros disponibles: Jazz, Pop, Rock, Rap, Otro.")
        }
    }

    /// Como en esta práctica no podemos usar Traits (salvo Debug), hacemos el Eq a mano.
    pub fn mismo_genero(&self, otro_genero:&Genero) -> bool {
        match self{
            Genero::Jazz => match otro_genero {
                Genero::Jazz => return true,
                _ => return false,
            }
            Genero::Otro => match otro_genero {
                Genero::Otro => return true,
                _ => return false,
            }
            Genero::Pop => match otro_genero {
                Genero::Pop => return true,
                _ => return false,
            }
            Genero::Rap => match otro_genero {
                Genero::Rap => return true,
                _ => return false,
            }
            Genero::Rock => match otro_genero {
                Genero::Rock => return true,
                _ => return false,
            }
        }   
    }

    /// Devuelve el nombre del género que representa en formato String.
    pub fn to_string(&self) -> String {
        match self{
            Genero::Jazz => return "Jazz".to_string(),
            Genero::Otro => return "Otro".to_string(),
            Genero::Pop => return "Pop".to_string(),
            Genero::Rap => return "Rap".to_string(),
            Genero::Rock => return "Rock".to_string(),
        }   
    }

}

#[derive(Debug)]
struct Playlist{
    titulo: String,
    lista: Vec<Cancion>,
}

impl Playlist {

    /// Crea una nueva lista de reproducción.
    pub fn new (nombre:String) -> Playlist{
        let vec_pl: Vec<Cancion> = Vec::new();
        Playlist { titulo: nombre, lista: vec_pl }
    }

    /// Agrega canción al final de la lista de reproducción.
    pub fn agregar_cancion(&mut self, cancion:Cancion){
        self.lista.push(cancion);
    }

    /// Busca el índice en el cual se encuentra una determinada canción.
    /// Si no se encuentra esa canción devuelve None.
    pub fn buscar_indice(&self, cancion:&Cancion) -> Option<usize>{
        for i in 0..self.lista.len() {
            if self.lista[i].misma_cancion(&cancion){
                return Some(i);
            }
        }
        return None;
    }

    /// Elimina una canción de la lista de reproducción.
    /// Devuelve True si la canción fue eliminada con éxito, False caso contrario.
    pub fn eliminar_cancion(&mut self, cancion:Cancion) -> bool {
        match self.buscar_indice(&cancion){
            Some(indice) => { 
                self.lista.remove(indice);
                return true;
            }
            _ => return false,
        }
    }

    /// Mueve la canción a una determinada posición de la lista de reproducción.
    /// Si la canción o la posición ingresadas no son válidas devuelve False.
    pub fn mover_cancion(&mut self, cancion:Cancion ,posicion:usize) -> bool{
        if let Some(indice_cancion) = self.buscar_indice(&cancion){
            if posicion < self.lista.len(){
                self.lista.remove(indice_cancion);
                self.lista.insert(posicion, cancion);
                return true;
            }else{
                return false;
            }
        }else{
            return false;
        }
    }

    /// Como lo indica el nombre, busca canción por nombre.
    /// Si no coincide el nombre devuelve None.
    pub fn buscar_canción_por_nombre (&self, nombre:String) -> Option<Cancion>{
        for i in 0..self.lista.len() {
            if self.lista[i].mismo_titulo(&nombre){
                return Some(self.lista[i].copiar());
            }
        }
        return None;
    }

    /// Crea una lista de reproducción sólo con las canciones de un determinado género.
    pub fn obtener_canciones_por_genero (&self, genero:Genero) -> Option<Playlist> {
        let mut playlist_genero:Vec<Cancion> = Vec::new();
        for i in 0..self.lista.len() {
            if self.lista[i].genero.mismo_genero(&genero){
                playlist_genero.push(self.lista[i].copiar());
            }
        }
        if playlist_genero.is_empty() {
            return None;
        } else {
            let titulo_genero:String = self.titulo.clone() + " ~ " + &genero.to_string();
            return Some(Playlist{ titulo: titulo_genero, lista: playlist_genero });
        }
    }

    // Crea una lista de reproducción sólo con las canciones de un determinado artista.
    pub fn obtener_canciones_por_artista (&self, artista:String) -> Option<Playlist> {
        let mut playlist_artista:Vec<Cancion>=Vec::new();
        for i in 0..self.lista.len() {
            if self.lista[i].mismo_artista(&artista){
                playlist_artista.push(self.lista[i].copiar());
            }
        }
        if playlist_artista.is_empty() {
            return None;
        } else {
            let titulo_artista:String = self.titulo.clone() + " ~ " + &playlist_artista[0].artista;
            return Some(Playlist{ titulo: titulo_artista, lista: playlist_artista });
        }
    }

    /// Modificar título de la lista de reproducción.
    pub fn cambiar_titulo (&mut self, nuevo_titulo:String){
        self.titulo = nuevo_titulo;
    }

    /// Eliminar todas las canciones.
    pub fn borrar_todo(&mut self){
        self.lista.clear();
    }

}

pub fn ejercicio8(){

        //main test
        let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());
    
        playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(),"2Pac".to_string(), "rap".to_string()));
        println!("{:?}", playlist);
        println!("...\n");
    
        println!("Eliminar");
        let cancion_eliminar:Cancion = Cancion::new("Words of Wisdom".to_string(),
         "2Pac".to_string(), "rap".to_string());
        assert_eq!(playlist.eliminar_cancion(cancion_eliminar), true);
    
        let cancion_eliminar_2:Cancion = Cancion::new("Hit 'em up".to_string(),
         "2Pac".to_string(), "rap".to_string());
        assert_eq!(playlist.eliminar_cancion(cancion_eliminar_2), false);
    
        println!("...\n");
    
        let cancion_2:Cancion = Cancion::new("Bitches Brew".to_string(),
          "Miles Davis".to_string(), "jazz".to_string());
        let cancion_3:Cancion = Cancion::new("When Doves Cry".to_string(),
         "Prince".to_string(), "pop".to_string());
        let cancion_4:Cancion = Cancion::new("Dolly Dagger".to_string(),
         "Jimi Hendrix".to_string(), "rock".to_string());
        let cancion_5:Cancion = Cancion::new("Crosstown Traffic".to_string(),
         "Jimi Hendrix".to_string(), "rock".to_string());
        let cancion_6:Cancion = Cancion::new("Computer Blues".to_string(),
         "Prince".to_string(), "otro".to_string()); 
        let cancion_7:Cancion = Cancion::new("Paisley Park".to_string(),
         "Prince".to_string(), "rock".to_string());
    
        playlist.agregar_cancion(cancion_2);
        playlist.agregar_cancion(cancion_3);
        playlist.agregar_cancion(cancion_4);
        playlist.agregar_cancion(cancion_5);
        playlist.agregar_cancion(cancion_6);
        playlist.agregar_cancion(cancion_7);
    
        println!("{:?}", playlist);
    
        if let Some(cancion) = playlist.buscar_canción_por_nombre("Crosstown Traffic".to_string()){
            println!("Mover la canción Crosstown Traffic a la posición 0 . . .");
            println!("{}\n", playlist.mover_cancion(cancion, 0));
        }else{
            println!("No se encontró.\n");
        }
    
        println!("{:?}", playlist);
    
        println!("...");
        let palomas:String = "When Doves Cry".to_string();
        println!("Buscar {}",palomas);
        println!("{:?}\n", playlist.buscar_canción_por_nombre(palomas));
    
        println!("...");
        println!("Obtener las canciones de un determinado género");
        println!("{:?}\n", playlist.obtener_canciones_por_genero(Genero::Rock));
    
        println!("...");
        println!("Obtener las canciones de un determinado artista");
        println!("{:?}\n", playlist.obtener_canciones_por_artista("Prince".to_string()));
    
        playlist.cambiar_titulo("Los más grandes".to_string());
        println!("{:?}\n", playlist);
        playlist.borrar_todo();
        println!("Borrar todas las canciones . . . {:?}", playlist);
        
    }
    

#[test]
fn test_agregar_cancion(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));

    assert_eq!(playlist.titulo, "Playlist 1".to_string());
    assert!(playlist.lista[0].misma_cancion(&Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string())));
}
#[test]
fn test_eliminar_cancion(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(playlist.eliminar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string())));
    assert_eq!(playlist.lista.len(), 2);
    assert!(playlist.lista[0].misma_cancion(&Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string())));
    assert!(playlist.lista[1].misma_cancion(&Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string())));
}

#[test]
fn test_eliminar_cancion_fallo(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(!playlist.eliminar_cancion(Cancion::new("Blue Monk".to_string(), "Thelonious Monk".to_string(), "jazz".to_string())));
    assert_eq!(playlist.lista.len(), 3);
}

#[test]
fn test_mover_cancion(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(playlist.mover_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()), 2));

    assert!(playlist.lista[0].misma_cancion(&Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string())));
    assert!(playlist.lista[1].misma_cancion(&Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string())));
    assert!(playlist.lista[2].misma_cancion(&Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string())));

}

#[test]
fn test_mover_cancion_fallo_indice(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(!playlist.mover_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()), 10));

}

#[test]
fn test_mover_cancion_fallo_cancion(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(!playlist.mover_cancion(Cancion::new("Blue Monk".to_string(), "Thelonious Monk".to_string(), "jazz".to_string()), 2));

}

#[test]
fn test_buscar_cancion(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(playlist.lista[1].misma_cancion(&playlist.buscar_canción_por_nombre("BITChES bRew".to_string()).unwrap()));
}

#[test]
fn test_buscar_cancion_fallo(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(playlist.buscar_canción_por_nombre("BLUE monk".to_string()).is_none());
}

#[test]
fn test_obtener_canciones_por_genero(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(playlist.lista[1].misma_cancion(&playlist.obtener_canciones_por_genero(Genero::Jazz).unwrap().lista[0]));
    assert_eq!(playlist.obtener_canciones_por_genero(Genero::Jazz).unwrap().titulo, "Playlist 1 ~ Jazz".to_string());
}

#[test]
fn test_obtener_canciones_por_genero_fallo(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(playlist.obtener_canciones_por_genero(Genero::Otro).is_none());
}

#[test]
fn test_obtener_canciones_por_artista(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(playlist.lista[0].misma_cancion(&playlist.obtener_canciones_por_artista("2PAC".to_string()).unwrap().lista[0]));
    assert_eq!(playlist.obtener_canciones_por_artista("2PAC".to_string()).unwrap().titulo, "Playlist 1 ~ 2Pac".to_string());
}

#[test]
fn test_obtener_canciones_por_artista_fallo(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    assert!(playlist.obtener_canciones_por_artista("Thelonious Monk".to_string()).is_none());
}

#[test]
fn test_cambiar_titulo(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());
    playlist.cambiar_titulo("Nueva Playlist!".to_string());

    assert_eq!(playlist.titulo, "Nueva Playlist!".to_string());
}

#[test]
fn test_borrar_todo(){

    let mut playlist: Playlist = Playlist::new("Playlist 1".to_string());

    playlist.agregar_cancion(Cancion::new("Words of Wisdom".to_string(), "2Pac".to_string(), "rap".to_string()));
    playlist.agregar_cancion(Cancion::new("Bitches Brew".to_string(), "Miles Davis".to_string(), "jazz".to_string()));
    playlist.agregar_cancion(Cancion::new("When Doves Cry".to_string(), "Prince".to_string(), "pop".to_string()));

    playlist.borrar_todo();

    assert!(playlist.lista.is_empty());
}