//ESEMPIO DELL'ESERCIZIO Listing 5-14: Using the as-yet-unwritten  can_hold method
//SI TROVA IN : can_old\src\main.rs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { //parametri sola lettura se vengono passati con rect4.area()); senza &
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn run() {
    println!(
        "\n
    //   I° ESERCIZIO DEL PARAGRAFO   5.3.03_Methods with More Parameters_Metodi con più parametri
    //-----------------------------------------------------------------------------------------//
    Note: l'esercizio si trova in: 
    Listato 5-14, una volta definito il metodo can_hold.
    //  {{C://CASA//PROGRAMMI//RUST_PROGETTI//Capitolo_05//can_old//src//main.rs}}
      
    
    Questo esercizio si basa sul METODO CON PIU PARAMETRI
    Viene creata un'istanza di Rettangolo che  prenda un'altra istanza di Rettangolo e 
    restituisca true se il secondo Rettangolo può adattarsi completamente all'interno di del primo; 
    altrimenti dovrebbe restituire false. 
    
    
    "
    );

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

    //per chiamare la funzine con 1 parametro corre sempre qualificare la variabile
    //oggetto e chiamarla con questo sistema di lettura : rect4.area());
    let rect4= Rectangle{
        width: 10,
        height: 20,
    };

    //todo: da chiamare 
    println!("\n
    AREA DEL QUADRATO  con 1 parametro in SOLA LETTURA  mediante questo tipo di istruzione:
        fn area(&self) -> u32 {{
            self.width * self.height
        }}
        costruisce questa area:     
    area rettangolo con  1 parametro {}", rect4.area());
    println!("\n
    AREA DEL RETTANGOLO 2 CON PARAMETRI E CON RISULTATO MODIFICABILE: 
        FUNZIONE: ---->   fn can_hold(&self, other: &Rectangle) -> bool {{
                            self.width > other.width && self.height > other.height
                        }}
    DA QUESTO RISULTATO                 
    Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("\n
    AREA DEL RETTANGOLO 2 CON PARAMETRI E CON RISULTATO MODIFICABILE: 
    come sopra con questo risultato:
    Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
} //main
