#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //metodo non statico perche richiede l'istanza con &self e bisogna chiamrla var.can_hold(&rect)
    //in Java era public boolean can_hold(Rectangle other)
    fn can_hold(&self, other: &Rectangle) -> bool {
        //attenzione return è opzionale ma devi togliere il ;
        return self.width > other.width && self.height > other.height;
    }
}

//per utilizzarla la struttura del rettangolo usare impl
impl Rectangle {
    // metodo statico non richiede l'istanza ma basta il nome del tipo es. Rectangle::square(20)
    // in java era public static Rectangle square (int size)
    fn square(size: u32) -> Rectangle {
        return Rectangle {//utilizzo Rectangle per restituire un oggetto Rectangle che rappresenta un quadrato
            width: size,    //in quanto ricevento un solo parametro u32 si valorizza la larghezza e l'altezza
            height: size,   //CON LO STESSO VALORE lxl = quadrato.
        };
    }
}

pub fn run() {
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
 
   let rect3 = Rectangle {
        width: 60,
        height: 45,
    };




    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
  
    //ATTENZIONE occorre il formattatore {:?} per una riga {:#?} su piu righe 
    //stampa l'intera struttura Rectangle con i campi ed i valori.
    // in Java era cosi Rectangle.square(3);
    println!("crea un quadrato con il METODO STATICO {:#?}", Rectangle::square(3));
  

}//main
