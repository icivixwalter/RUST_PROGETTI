
#[derive(Debug)]
#[allow(dead_code)] // annotazione che permette l'utilizzo del codice non utilizzato
struct Rectangle {
    width: u32,         //todo : di sola lettura come eliminare il warning??
    height: u32,
}
pub fn run() {
    println!(
        "
    5.2.03_Adding Useful Functionality with Derived Traits_Aggiunta 
    di funzionalità utili con tratti derivati (LE INTERFACCE)
//===============================================================================================//
NOTE : esercizio relativo all'utilizzo della macro debug - con la preinstazione della seguente
aggiunta:
    ---> #[derive(Debug)]
l'inclusione di tale macro permette l'utilizzo del comando dbg!(.....) il quale permette di 
costruire una formattazione di debug utilizzando lo standard della console degli errori:
    ---> errore standard (stderr), 
a differenza di println!  che  stampa sul flusso della console di output standard (stdout).
Per evitare che dbg! ACQUISTI LA PROPRIETA

dbg! viene utilizzato per visualizzare il debug, infatti viene stampata la riga 30

//===============================================================================================//

    "
    );

   
        let scale = 2;

        println!("Qui stampa la riga 30 con tutto il risultato
                        ----> width: dbg!(30 * scale) \n");
        let rect1 = Rectangle {
           width: dbg!(30 * scale), //qui dbg stampa anche la riga 30 con il valore
            height: 50,
        };
println!("\n
//STAMPA L'INTERA STRUTTURA CON IL COMANDO DBG!
//---------------------------------------------------------------------------//
stampo l'intera struttura dati della struct con il comando dbg! insieme ai valori. \n");
        dbg!(&rect1); //utilizzando dbg!(&rect1) utilizzo il riferimento e non ottengo la proprieta
    
}
