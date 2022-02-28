//defining_methods\src\main.rs

#[derive(Debug)]
struct Rectanglenw { //definzione di un nome = ad un TIPO DI DATO
    width: u32,
    height: u32,
}

//I° ESEMPIO di creazione di un metodo
impl Rectanglenw {
    //utilizzo &self = this in java
    //self: &Self è stato abbreviato con &self di tipo Self. Viene usato &self
    //perchè si vuole avere solo una lettura ed è un prestito di proprieta DI SOLA LETTURA.
    fn areanew(&self) -> u32 {//QUESTA E' LA FIRMA DEL METODO
        self.width * self.height //implemtanzione del metodo
    }
}

// region: main
pub fn run() {
	/* VARIABILE OGGETTO rect1 =  quando viene istanziata avvengono due procedimenti,
		1) istanzio la struttura struct Rectanglenw;
		2) attivo l'implementazione impl Rectanglenw e di conseguenza
			si attiva la funzione   fn areanew(&self) -> u32 con il 
			conseguente calcolo dell'area e restituzione del valore alla variabile
			proprietari rect1

		- Per attivare i due procedimenti occorre richiamare nel codice la variabili di istanza:
				rect1.areanew()
	 */
    let rect1 = Rectanglenw {
        width: 30,
        height: 50,
    };

    println!("

    //  I° ESEMPIO: 5.3.01_Defining Methods_Definizione di metodi                                       //
    ------------------------------------------------------------------------------------------------------
    Cambiamo la funzione areanew che ha un'istanza Rectanglenw 
    come parametro e creiamo invece un metodo areanew definito sulla struttura Rectanglenw, 
	come mostrato nel Listato 5-13.
      
        ");
    println!(
        "\n	
			IL CALCOLO DELL'AREA DEL RETTANGOLO E' IL SEGUENTE:
			-------------------------------------------------------------------
			The area of the Rectanglenw is {} square pixels.",
        rect1.areanew()
    );


    println!("\n\n\
    ---------------------------------------------------------------------------
                    FINE Capitolo_05 - defining_methods - \n\n");



} //FINE MAIN
  // endregion: main
