//ESEMPIO DELL'ESERCIZIO Listing 5-14: Using the as-yet-unwritten  can_hold method
//SI TROVA IN : can_old\src\main.rs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
} //main
