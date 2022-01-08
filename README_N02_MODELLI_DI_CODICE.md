# esempi rust book - ita


# MODELLO DI CHIAMATA DELLA FUNZIONE

## 01) CREARE L'OGGETTO BUTTON
Il button viene creato per essere inserito nella gui di
rust. Viene creato ul bottone ed una funzione

    //ATTENZIONE: deve essere rinominato il file main.rs a lib rs del progetto ownership che deve
    //essere incluso nel file cargo.toml del progetto generale inoltre nel file lib.rs occorre
    //rinominare la funzione principale main ---> run con tipo public: pub fn run() {...
    //---------------------------------------------------------------------------------------//
        //[nwg_control 	= creo un nuovo controllo con le proprieta text, size, position
        #[nwg_control(text: "Cap_04 - ownership - bt_08", 
        size: (280, 30),    		//larg + alt del bottone 
        position: (10, 450))]		//la posizione 10 X posizione + 50 Y
        //nwg_events  	= attivo l'evento del controllo con la funzione OnButtonClick
        //			la quale chiama la implementazione BasicApp e la funzione creata.
        #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_08] )]
        //creo il nuovo oggetto button con la libreria nwg
        button_08: nwg::Button,		
    //---------------------------------------------------------------------------------------//



## 02) CREARE LA FUNZIONE PER IL L'EVENTO BUTTON

    //attivo la funzione con il parametro &self = this o se stesso
    fn fn_button_esercizio_08(&self) { //button_08 - progetto ---> ownership
        ownership::run(); 
    }


