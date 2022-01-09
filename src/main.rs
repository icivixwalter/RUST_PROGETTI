/*!
    A very simple application that show your name in a message box.
    See `basic` for the version without the derive macro
*/

extern crate data_type;
extern crate guessing_game;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
// use std::{sync::Mutex, thread};
// use std::sync::atomic::AtomicBool;
// static spawned: Mutex<bool> = Mutex::new(false);

//I° FILA DI 10 BOTTONI - DA 1-10
const FILA_01_BUTTON_SIZE: (i32, i32) = (310, 30);       //larghezza + altezza fissi

//II° FILA DI 10 BOTTONI - DA 11-20 *** da utilizzare ***
const FILA_02_BUTTON_SIZE: (i32 , i32) = (310, 30);       //larghezza + altezza fissi

//LA FINESTRA WINDOWS
const DIMENSIONI_WINDOWS:(i32, i32)=(800,600);    // (800,600)---> larghezza e posizione della finestra windows con 20 bottoni
//con 10 bottoni posizione e la grandezza della finestra windows era di (600, 635)
//con 2 bottoni posizione originale della finestra windows (300,135)



#[derive(Default, NwgUi)]
pub struct BasicApp {
    //CREA LA FINESTRA WINDOWS CON LE DIMENSIONI E LA POSIZIONE
    //---------------------------------------------------------------------------------------//
    //controllo base è questo size: (300, 135), position: (300, 300), per un bottone
    //la posizione base è questa position: (300, 300) cambio in ----> 300,10
    //paer avere la finestra centrale
    //con 600,435 entrano = 2 bottoni
    //ORIGINALE ----> #[nwg_control(size: (600, 635), position: (300, 10), //cambio in ----> 300,10
    #[nwg_control(size: DIMENSIONI_WINDOWS, position: (300, 10), //cambio in ----> 300,10
    
                title: "Basic example", 
                flags: "WINDOW|VISIBLE")]

    //EVENTO DI CHIUSURA DELL'APPLICAZIONE.
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )] //evento messaggio 02
    window: nwg::Window,
    //---------------------------------------------------------------------------------------//

    




//---------------------------------------------------------------------------------------//


    //CASELLA DI TESTO Con la stringa - EDITABILE
    //---------------------------------------------------------------------------------------//
    //originale:
    // #[nwg_control(text: "Heisenberg", size: (280, 35), position: (10, 10), focus: true)]
    #[nwg_control(text: "Hello ", 
                size: (230, 35),                    //casella di testo largh + alt originale 280,35 ---- > 230,35
                position: (10, 10), focus: true)] //casella posizione x,y
    name_edit: nwg::TextInput,
    //---------------------------------------------------------------------------------------//

    //BOTTONE 1 CON L'ETICHETTA
    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "Say my name", 
                size: (230, 30),    //larg + alt del bottone da 280, 70 ---> ridotto 280, 30
                position: (10, 50))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button, //BOTTONE 1 CON L'ETICHETTA
    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "01) Cap 1 - Run BAT", 
    size: FILA_01_BUTTON_SIZE,    //I° FILA DI 10 BOTTONI: larg + alt       del bottone 
    position: (10, 100))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_01] )]
    button_01: nwg::Button,
    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "02) Cap 2 - Guessing Game", 
        size: FILA_01_BUTTON_SIZE,    //I° FILA DI 10 BOTTONI: larg + alt       del bottone 
        position: (10, 150))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_02] )]
    button_02: nwg::Button,
    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "03) Cap 3 - Data type", 
        size: FILA_01_BUTTON_SIZE,    //I° FILA DI 10 BOTTONI: larg + alt       del bottone 
        position: (10, 200))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_03] )]
    button_03: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "04) Cap 3 - function", 
    size: FILA_01_BUTTON_SIZE,    //I° FILA DI 10 BOTTONI: larg + alt       del bottone 
    position: (10, 250))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_04] )]
    button_04: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "05) Cap 3 - progetto loop", 
    size: FILA_01_BUTTON_SIZE,    //I° FILA DI 10 BOTTONI: larg + alt       del bottone 
    position: (10, 300))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_05] )]
    button_05: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "06) Cap 3 - variabili & mutability", 
    size: FILA_01_BUTTON_SIZE,    //I° FILA DI 10 BOTTONI: larg + alt       del bottone 
    position: (10, 350))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_06] )]
    button_06: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "07) Cap_04 - dangle", 
    size: FILA_01_BUTTON_SIZE,    //I° FILA DI 10 BOTTONI: larg + alt       del bottone 
    position: (10, 400))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_07] )]
    button_07: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //ATTENZIONE: deve essere rinominato il file main.rs a lib rs del progetto ownership che deve
    //essere incluso nel file cargo.toml del progetto generale inoltre nel file lib.rs occorre
    //rinominare la funzione principale main ---> run con tipo public: pub fn run() {...
    //---------------------------------------------------------------------------------------//
    //[nwg_control 	= creo un nuovo controllo con le proprieta text, size, position
    #[nwg_control(text: "08) Cap_04 - ownership", 
	size: FILA_01_BUTTON_SIZE,    		//I° FILA DI 10 BOTTONI: larg + alt         del bottone 
	position: (10, 450))]
    //la posizione 10 X posizione + 50 Y
    //nwg_events  	= attivo l'evento del controllo con la funzione OnButtonClick
    //			la quale chiama la implementazione BasicApp e la funzione creata.
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_08] )]
    //creo il nuovo oggetto button con la libreria nwg
    button_08: nwg::Button,
    //---------------------------------------------------------------------------------------//

    
    //ATTENZIONE: deve essere rinominato il file main.rs a lib rs del progetto references_and_borrowing che deve
    //essere incluso nel file cargo.toml del progetto generale inoltre nel file lib.rs occorre
    //rinominare la funzione principale main ---> run con tipo public: pub fn run() {...
    //---------------------------------------------------------------------------------------//
        //[nwg_control 	= creo un nuovo controllo con le proprieta text, size, position
        #[nwg_control(text: "09) Cap_04 - references_and_borrowing", 
        size: FILA_01_BUTTON_SIZE,    		//I° FILA DI 10 BOTTONI: larg CON LA COSTANTE + alt del bottone NUMERICA
        position: (10, 500))]		        //la posizione 10 X posizione + 50 Y
        //nwg_events  	= attivo l'evento del controllo con la funzione OnButtonClick
        //			la quale chiama la implementazione BasicApp e la funzione creata.
        #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_09] )]
        //creo il nuovo oggetto button con la libreria nwg
        button_0: nwg::Button,		
    //---------------------------------------------------------------------------------------//

     //ATTENZIONE: deve essere rinominato il file main.rs a lib rs del progetto the_slice_type che deve essere incluso nel file cargo.toml del progetto generale inoltre nel file lib.rs occorre
    //rinominare la funzione principale main ---> run con tipo public: pub fn run() {...
    //---------------------------------------------------------------------------------------//
        //[nwg_control 	= creo un nuovo controllo con le proprieta text, size, position
        #[nwg_control(text: "10) Cap_04 - the_slice_type", 
        size: FILA_01_BUTTON_SIZE,    		//I° FILA DI 10 BOTTONI: larg CON LA COSTANTE + alt del bottone NUMERICA
        position: (10, 550))]		        //la posizione 10 X posizione + 50 Y
        //nwg_events  	= attivo l'evento del controllo con la funzione OnButtonClick
        //			la quale chiama la implementazione BasicApp e la funzione creata.
        #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_10] )]
        //creo il nuovo oggetto button con la libreria nwg
        button_10: nwg::Button,		
    //---------------------------------------------------------------------------------------//

    // region: II_FILA_BUTTON_DA_11-20
    //****************************************************************************************//
    //          II° FILA DI BOTTONI
   	
    //BUTTON_11 - II FILA DA 11 - 20
    //---------------------------------------------------------------------------------------//
        #[nwg_control(text: "11) Cap_05 - can_old", 
        //le dimensioni della Button è impostato in modo fissa dalla COSTANTE.
        size: FILA_02_BUTTON_SIZE,    //II° FILA DI 10 BOTTONI: larg + alt       del bottone 
        /*POSIZIONE DEL BUTTON è incrementale di 2,4,6,8,10 ecc.. partendo dalla posizione 100 
            si ha una automatica disposizione della casella text  INCREMENTANDO DI UN MULTIPLO
            DI 2 - Oggetto Button per il progetto - can_old
        */
        position: (350, 100+(0*25)))]  //posizione incrementale dell'oggetto button, basta cambiare lo 0 con                            //un multiplo di due
        

        #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_11] )]
        button_11: nwg::Button,
    //---------------------------------------------------------------------------------------//
    
    
    
    //BUTTON_12 - II FILA DA 11 - 20 - PROGETTO ---> defining_methods
    //---------------------------------------------------------------------------------------//
        #[nwg_control(text: "12) Cap_05 - defining_methods", 
        //le dimensioni della Button è impostato in modo fissa dalla COSTANTE.
        size: FILA_02_BUTTON_SIZE,    //II° FILA DI 10 BOTTONI: larg + alt       del bottone 
        /*POSIZIONE DEL BUTTON è incrementale di 2,4,6,8,10 ecc.. partendo dalla posizione 100 
            si ha una automatica disposizione della casella text  INCREMENTANDO DI UN MULTIPLO
            DI 2 - Oggetto Button per il progetto - defining_methods
        */
        position: (350, 100+(2*25)))]  //posizione incrementale dell'oggetto button, basta cambiare lo 0 con                            //un multiplo di due
        

        #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_12] )]
        button_12: nwg::Button,
    //---------------------------------------------------------------------------------------//

    


    //BUTTON 13) - II FILA DA 11 - 20 - PROGETTO ---> format_debug
    //---------------------------------------------------------------------------------------//
        #[nwg_control(text: "13) Cap_05 - format_debug", 
        //le dimensioni della Button è impostato in modo fissa dalla COSTANTE.
        size: FILA_02_BUTTON_SIZE,    //II° FILA DI 10 BOTTONI: larg + alt       del bottone 
        /*POSIZIONE DEL BUTTON è incrementale di 2,4,6,8,10 ecc.. partendo dalla posizione 100 
        si ha una automatica disposizione della casella text  INCREMENTANDO DI UN MULTIPLO
        DI 2 - Oggetto Button per il progetto - format_debug
        */
        position: (350, 100+(4*25)))]  //posizione incrementale dell'oggetto button, basta cambiare lo 0 con                            //un multiplo di due


        #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_13] )]
        button_13: nwg::Button,
    //---------------------------------------------------------------------------------------//

//BUTTON 14) - II FILA DA 11 - 20 - PROGETTO ---> multiple_impl_blocks
//---------------------------------------------------------------------------------------//
    #[nwg_control(text: "14) Cap_05 - multiple_impl_blocks", 
    //le dimensioni della Button è impostato in modo fissa dalla COSTANTE.
    size: FILA_02_BUTTON_SIZE,    //II° FILA DI 10 BOTTONI: larg + alt       del bottone 
    /*POSIZIONE DEL BUTTON è incrementale di 2,4,6,8,10 ecc.. partendo dalla posizione 100 
    si ha una automatica disposizione della casella text  INCREMENTANDO DI UN MULTIPLO
    DI 2 - Oggetto Button per il progetto - multiple_impl_blocks
    */
    position: (350, 100+(6*25)))]  //posizione incrementale dell'oggetto button, basta cambiare lo 0 con                            //un multiplo di due


    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_14] )]
    button_14: nwg::Button,
//---------------------------------------------------------------------------------------//


    //BUTTON 15) - II FILA DA 11 - 20 - PROGETTO ---> operator_c
    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "15) Cap_05 - operator_c", 
    //le dimensioni della Button è impostato in modo fissa dalla COSTANTE.
    size: FILA_02_BUTTON_SIZE,    //II° FILA DI 10 BOTTONI: larg + alt       del bottone 
    /*POSIZIONE DEL BUTTON è incrementale di 2,4,6,8,10 ecc.. partendo dalla posizione 100 
    si ha una automatica disposizione della casella text  INCREMENTANDO DI UN MULTIPLO
    DI 2 - Oggetto Button per il progetto - operator_c
    */
    position: (350, 100+(8*25)))]  //posizione incrementale dell'oggetto button, basta cambiare lo 0 con                            //un multiplo di due


    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_15] )]
    button_15: nwg::Button,
//---------------------------------------------------------------------------------------//






    
    //****************************************************************************************//
    // endregion: II_FILA_BUTTON_DA_11-20   
    
   

}

//le funzioni esterne
impl BasicApp {
    //buttone 1
    fn fn_button_esercizio_01(&self) {
       // call_exe::call("APRI_FILE_{guessing_game}.bat");
       //2 ° call con path
       call_exe::call("c:\\CASA\\PROGRAMMI\\RUST_PROGETTI\\Capitolo_01\\APRI_FILE_{HelloWord_eseguibile}.bat");
    }

//region : I° FILA DELLA FUNZIONI BUTTON
    //buttone 2
    fn fn_button_esercizio_02(&self) {
        guessing_game::game(); // TODO spawna un thread che esegue game()
    }

    fn fn_button_esercizio_03(&self) {
        data_type::run();
    }

    fn fn_button_esercizio_04(&self) {
        function::run();
    }

    fn fn_button_esercizio_05(&self) {
        progetto_loop::run();
    }

    fn fn_button_esercizio_06(&self) {
        variables_and_mutability::run(); //button 06
    }

    fn fn_button_esercizio_07(&self) {
        //button 07 - dangle
        dangle::run();
    }

    //attivo la funzione con il parametro &self = this o se stesso
    fn fn_button_esercizio_08(&self) {
        //button_08 - progetto ---> ownership
        ownership::run();
    }

     //attivo la funzione con il parametro &self = this o se stesso
     fn fn_button_esercizio_09(&self) { //button_09 - progetto ---> references_and_borrowing
        references_and_borrowing::run(); 
    }

    //attivo la funzione con il parametro &self = this o se stesso
    fn fn_button_esercizio_10(&self) { //button_10 - progetto ---> the_slice_type
        the_slice_type::run(); 
    }

    //endregion : II° FILA DELLA FUNZIONI BUTTON

//region : II° FILA DELLA FUNZIONI BUTTON
// II° FILA DI BUTTON
//-------------------------------------------------------------------------------//
    
   //attivo la funzione con il parametro &self = this o se stesso
   fn fn_button_esercizio_11(&self) { //button_11 - progetto ---> can_old
    can_old::run(); 
    }

    //attivo la funzione con il parametro &self = this o se stesso
    fn fn_button_esercizio_12(&self) { //button_12 - progetto ---> defining_methods
        defining_methods::run(); 
    }

    //attivo la funzione con il parametro &self = this o se stesso
    fn fn_button_esercizio_13(&self) { //button_13 - progetto ---> format_debug
        format_debug::run(); 
    }

    
   //attivo la funzione con il parametro &self = this o se stesso
   fn fn_button_esercizio_14(&self) { //button_14 - progetto ---> multiple_impl_blocks
    multiple_impl_blocks::run(); 
    }

       //attivo la funzione con il parametro &self = this o se stesso
       fn fn_button_esercizio_15(&self) { //button_15 - progetto ---> operator_c
        operator_c::run(); 
    }


//-------------------------------------------------------------------------------//
//endregion : II° FILA DELLA FUNZIONI BUTTON

    //messaggio_01
    fn say_hello(&self) {
        nwg::modal_info_message(
            &self.window,
            "Esempio message box",
            &format!("Hello {}", self.name_edit.text()),
        );
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(
            &self.window,
            "MESSAGGIO Goodbye",
            &format!("Goodbye {}", self.name_edit.text()),
        );

        //tread stop
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    //SE FALLISCE LA COSTRUZIONE DELLA GUI MSG
    nwg::init().expect("Failed to init Native Windows GUI");
    //QUI METTE il tipo di testo ??
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    //EVENTI
    nwg::dispatch_thread_events();
}

// region:modello_originale
/*
modello originale:
=========================================================================================
/*!
    A very simple application that show your name in a message box.
    See `basic` for the version without the derive macro
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 135), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text: "Heisenberg", size: (280, 35), position: (10, 10), focus: true)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Say my name", size: (280, 70), position: (10, 50))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button
}

impl BasicApp {

    fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name_edit.text()));
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

*/

// endregion:modello_originale
