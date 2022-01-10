//use std::io::stdin; //libreria io

//per usare la funzione del calcolo aree

mod funzioni_calcolo_rettangolo;
use crate::funzioni_calcolo_rettangolo::area;
//macro di RUST che permette di creare un metodo automatico per la stampa della struct
//come il toString di Java
#[derive(Debug)] //annotazione che in java si fa con la chiocciola @
struct Rectangle {
    width2: u32,
    height2: u32,
}

// region: main
pub fn main() {
    println!("AREA DEL RETTANGOLO\n");

    //imposto le variabili per l'area
    let width1 = 30;
    let height1 = 50;

    println!(
        //STAMPO recuperando come parametro l'area calcolata e la funzione è esterna
        "
            I° ESEMPIO di funzione area con 2 parametri esterna
        //================================================================================================//
        CHIAMO LA FUNZIONE ESTERNA PER IL CALCOLO DELL'AREA:
             The area of the rectangle is {} square pixels.\n\n",
        area(width1, height1)
    );

    //              ESEMPIO II PRESTITO
    //---------------------------------------------------------------------------//
    //istanza rect1 data in prestito a Rectangle

    let rect2 = Rectangle {
        //main mantiene la proprieta dell’istanza di rect1
        width2: 30,
        height2: 50,
    };

    println!( "\n\n 
                II° ESEMPIO FUNZIONE CHE PRENDE IN PRESTITO LA STRUTTURA
        //================================================================================================//
        NUOVA FUNZIONE ESEMPIO DI PRESTITO DI STRUTTURA - PROPRIETA DI rect1
        la struttura dati impostata con 
        ---> struct Rectangle {{ width2: u32, height2: u32, }}
        viene prima assunta come proprieta  dalla variabile:
                ---> let rect2 = Rectangle {{......
        e presa in prestito mediante il puntatore a riferimento dalla funzione:
                ----> fn area(rectangle: &Rectangle) -> u32  ...
        IL risultato è che la funzione AREA prende in prestito con la proprieta i dati della struttura 
        che appartengono a rect1, elabora il calcolo e restituisce il risultato alla chiamata :
                ----> area(&rect1)...
        che stampa con la funzione print il risultato ma RESTITUISCE LA PROPRIETA alla variabile rect1 che
        rimane ancora valida

        The area of the rectangle is {} square pixels.\n", 
        area_2(&rect2) );

    println!("\n\n
        //   ULTIMA DIMOSTRAZIONE: USCITA DALL'AMBITO DI VALIDITA DEL PRINT MA rect2 è ancora valida         
        //================================================================================================//
        Per la stampa ho utilizzato la macro debug che crea il metodo automatico di stampa
        della struttura, infatti dimostro che la variabile proprietaria della struttura 
        &rect2 è ancora valida ed  il valore di \n
        VARIABILE PROPRIETARIA - &rect2 is : {:?}",&rect2);

    //---------------------------------------------------------------------------//

    //              RITARDO DELLA SCHELL
    //---------------------------------------------------------------------------//
    // utilizzare questa libreria:
    //'use::std::io;'
    //use std::io::stdin;

    // let mut s = String::new();
    // println!("\n\n RITARDO DELLA SCHELL: premi invio per uscire!");
    // stdin()
    //     .read_line(&mut s)
    //     .expect("Did not enter a correct string");

    //---------------------------------------------------------------------------//
}
// endregion: main

// region: funzioni_esterne
/*FUNZIONE RECTANGLE MIGLIORATA= contiene un solo parametro rectangle: &Rectangle costruita
    con il comando & (e commerciale) la quale permette il prestito dell'istanza Rectangle
*/
fn area_2(rectangle: &Rectangle) -> u32 {
    //funzione area modificata solo 1 parametro

    println!(
    "
    II° ESEMPIO
    ATTIVAZIONE FUNZIONE RECTANGLE SEMPLIFICATA utilizzo &
    //---------------------------------------------------------------------------//
    La funzione rectangle utilizza la & commerciale e permette di costruire una funzione 
    con 1 solo parametro fn area(rectangle: &Rectangle) -> u32 ...
    Viene utilizzata la tecnica del PRESTITO IMMUTABILE e  cioè viene preso in prestito
    il parametro Rectangle, la struttura ma non la proprieta della stessa che rimane in
    possesso del main chiamante.
    //---------------------------------------------------------------------------//"
    );

    // e esegue un prestito immutabile di una
    //istanza di rectangle = & permette il prestito
    rectangle.width2 * rectangle.height2
}
// endregion: funzioni_esterne

