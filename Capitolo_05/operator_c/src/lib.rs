//==================================================================================//
//ESEMPIO COMPLETO DI UTILIZZO  DEL RIFERIMENTO AUTOMATICO Where’s the -> Operator?
//si trova in: C:\CASA\PROGRAMMI\RUST_PROGETTI\Capitolo_05\operator_c\src\main.rs

#![allow(unused)]
pub fn run() {
    #[derive(Debug, Copy, Clone)]
    //crea una struttura point ma la firma e l'impmentazione è esterna.
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            //implemento la funzione distance
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);
            f64::sqrt(x_squared + y_squared)
        }

    }    
        //definisco ed assegno le due variabili Point all'esterno dell'implementazione
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 5.0, y: 6.5 };
        //attivo i due modi di richiedere i puntatori a p1 che sono uguali
        p1.distance(&p2);
        (&p1).distance(&p2);
    

    println!("\n
    //==================================================================================//
    //ESEMPIO COMPLETO DI UTILIZZO  DEL RIFERIMENTO AUTOMATICO Where’s the -> Operator?
    
    gli operatori in c sono due:
        methods:    se chiami direttamente l'oggetto
        operator -> se chiami il puntatore all'oggetto.
    Rust ha invece una funzione chiamata referenziamento e dereferenziazione automatici, per arrivare
    a questo risultato nell'esercizio vengono eseguiti i seguenti passaggi:
    1) creo un struttura denominata
            struct Point {{
                x: f64,
                y: f64,
            }}

    2) costruisco una implementazione della struttura point:
    impl Point {{
        //crea la firma della funzione distance
        fn distance(&self, other: &Point) -> f64 {{
            //implemnto la funzione distance 
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);
            //radice quadrata
            f64::sqrt(x_squared + y_squared)
        }}
    }}

    3)//definisco ed assegno le due variabili Point all'esterno dell'implementazione
        let p1 = Point {{ x: 0.0, y: 0.0 }};
        let p2 = Point {{ x: 5.0, y: 6.5 }};
        //attivo i due modi di richiedere i puntatori a p1 che sono uguali
        p1.distance(&p2);
        (&p1).distance(&p2);

    
    
    
    
    Il puntatore definito in modo diverso ma con i risultati uguali per quanto riguarda
    il calcolo della distanza è il seguente:
    a) puntatore 1 -> p1.distance(&p2)      = {}
    b) puntatore 1 -> (&p1).distance(&p2)   = {}
  
    

   ", {p1.distance(&p2)}, {(&p1).distance(&p2)} );
}
// endregion: funzioni_esterne
