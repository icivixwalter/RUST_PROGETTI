# esempi rust book - ita


# MODELLO DI CHIAMATA DELLA FUNZIONE

## 01) CREARE L'OGGETTO BUTTON
Il button viene creato per essere inserito nella gui di
rust. Viene creato ul bottone ed una funzione

    //ATTENZIONE: deve essere rinominato il file main.rs a lib rs del progetto the_slice_type che deve essere incluso nel file cargo.toml del progetto generale inoltre nel file lib.rs occorre
    //rinominare la funzione principale main ---> run con tipo public: pub fn run() {...
    //---------------------------------------------------------------------------------------//
        //[nwg_control 	= creo un nuovo controllo con le proprieta text, size, position
        #[nwg_control(text: "Cap_04 - the_slice_type - bt_10", 
        size: BUTTON_SIZE,    		//larg CON LA COSTANTE + alt del bottone NUMERICA
        position: (10, 550))]		//la posizione 10 X posizione + 50 Y
        //nwg_events  	= attivo l'evento del controllo con la funzione OnButtonClick
        //			la quale chiama la implementazione BasicApp e la funzione creata.
        #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_10] )]
        //creo il nuovo oggetto button con la libreria nwg
        button_10: nwg::Button,		
    //---------------------------------------------------------------------------------------//



## 02) CREARE LA FUNZIONE PER IL L'EVENTO BUTTON

    //attivo la funzione con il parametro &self = this o se stesso
    fn fn_button_esercizio_10(&self) { //button_10 - progetto ---> the_slice_type
        the_slice_type::run(); 
    }


## 03) CREARE BUTTON INCREMENTALE 
Per creare una fila di oggetti button con posizione automatica incrementale, occorre per prima cosa 
impostare le costanti sia della FINESTRA DI WINDOWS  e siA LA COSTANTE DELLE DIMENSIONI DEGLI OGGETTI BUTTON.

## LE COSTANTI PRELIMINARI DELLA LARGHEZZA E DELLA ALTEZZA DEI BUTTON INCREMENTALI
Per prima cosa occorre definire le costanti dei button incrementali e della finestra:
viene definita la costante che imposta le dimensioni largh+alte del button da utilizzare 
per tutti gli oggetti button
    //II° FILA DI 10 BOTTONI - DA 11-20 *** da utilizzare ***
    const FILA_02_BUTTON_SIZE: (i32 , i32) = (310, 30);       //larghezza + altezza fissi

    //LA FINESTRA WINDOWS
    const DIMENSIONI_WINDOWS:(i32, i32)=(800,600);    // (800,600)---> larghezza e posizione della finestra windows con 20 bottoni
    
### LA CREAZIONE DEL BUTTON INCREMENTALE CON LA COSTANTE PREDEFINITA
La seconda operazione è creare l'oggetto button incrementale la cui funzione di attivazione rimane la stessa
indicata sopra.

    //      II° FILA DI BUTTON 
    //============================================================================================================/
    
    //BUTTON 15) - II FILA DA 11 - 20 - PROGETTO ---> operator_c
    //---------------------------------------------------------------------------------------//
        #[nwg_control(text: "15) Cap_05 - operator_c", 
        //le dimensioni della Button è impostato in modo fissa dalla COSTANTE.
        size: FILA_02_BUTTON_SIZE,    //II° FILA DI 10 BOTTONI: larg + alt       del bottone 
        /*POSIZIONE DEL BUTTON è incrementale di 2,4,6,8,10 ecc.. partendo dalla posizione 100 
        si ha una automatica disposizione della casella text  INCREMENTANDO DI UN MULTIPLO
        DI 2 - Oggetto Button per il progetto - operator_c
        */
        position: (350, 100+(6*25)))]  //posizione incrementale dell'oggetto button, basta cambiare lo 0 con                            //un multiplo di due


        #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_15] )]
        button_15: nwg::Button,
    //---------------------------------------------------------------------------------------//

       
### LA FUNZIONE DEL BUTTON INCREMENTALE
la funzione che viene chiamata dal bottone incrementale:

     //attivo la funzione con il parametro &self = this o se stesso
    fn fn_button_esercizio_15(&self) { //button_15 - progetto ---> operator_c
        operator_c::run(); 
    }

### FILE TOML PER IL BUTTON INCREMENTALE
nel file toml occorre impostare la seguente dicitura per il bottone incrementale

    operator_c={path="Capitolo_05/operator_c"} #button_15 - c:\CASA\PROGRAMMI\RUST_PROGETTI\Capitolo_05\operator_c\




# STRUTTURE DATI
Costruire e chiamare una struttura dati.


    /*faq:@struttura.dati, @richiamare.una.struttura_dati,@come.richiamare.
          @costruire.una.struttura.dati
    */

    //STRUTTURA DATI - COME COSTRUIRLA E CHIAMARLA
    //-------------------------------------------------------------------------------------------//
    //Note: la struttura viene costruita con diversi campi
    una.struttura.dati
    //La struttura data viene costruita assegnando ad una variabile l'intera struttura:

        //struttura dati
            let user1 = User {
                email: String::from("someone@example.com"),
                username: String::from("someusername123"),
                active: true,
                sign_in_count: 1,
            };
        
        //Per chiamare la struttura dati occorre qualificare la variabile privata con l'assegnazione 
        //mediante let e poi utilizzare il seguente costrutto: variabile.elementoStruttura, es.
        println!("
                //      Ownership_2 of Struct Data --- ESEMPIO STRUTTURA USER --
                ----------------------------------------------------------------------------------------//
                Note: in questo esempio, viene costruita una struttura ed assegnata alla variabile user1,
                è possibile richiamare i componenti della struttura qualificando la variabile oggetto.elemento
                come in questo esempio:

                La struttura costruita --->         let user1 = User {{
                                email: String::from('someone@example.com'),
                                username: String::from('someusername123'),
                                active: true,
                                sign_in_count: 1,
                                }};
                Richiamo i singoli elementi:
                user1.email         :                {}
                user1.username      :                {}
                user.sign_in_count  :                {}",user1.email, user1.username, user1.sign_in_count);
            
    //-------------------------------------------------------------------------------------------//
        
   