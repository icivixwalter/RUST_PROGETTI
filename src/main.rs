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
const BUTTON_SIZE: (i32, i32) = (280, 30);
#[derive(Default, NwgUi)]
pub struct BasicApp {
    //CREA LA FINESTRA WINDOWS CON LE DIMENSIONI E LA POSIZIONE
    //---------------------------------------------------------------------------------------//
    //controllo base è questo size: (300, 135), position: (300, 300), per un bottone
    //la posizione base è questa position: (300, 300) cambio in ----> 300,10
    //paer avere la finestra centrale
    //con 600,435 entrano = 2 bottoni
    #[nwg_control(size: (600, 635), position: (300, 10), //cambio in ----> 300,10
                title: "Basic example", 
                flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )] //evento messaggio 02
    window: nwg::Window,
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
    #[nwg_control(text: "Cap 1 - Run BAT", 
    size: BUTTON_SIZE,    //larg + alt del bottone 
    position: (10, 100))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_01] )]
    button_01: nwg::Button,
    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "Cap 2 - Guessing Game", 
        size: BUTTON_SIZE,    //larg + alt del bottone 
        position: (10, 150))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_02] )]
    button_02: nwg::Button,
    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "Cap 3 - Data type", 
        size: BUTTON_SIZE,    //larg + alt del bottone 
        position: (10, 200))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_03] )]
    button_03: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "Cap 3 - function bt_04", 
    size: BUTTON_SIZE,    //larg + alt del bottone 
    position: (10, 250))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_04] )]
    button_04: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "Cap 3 - progetto loop - button 05", 
    size: BUTTON_SIZE,    //larg + alt del bottone 
    position: (10, 300))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_05] )]
    button_05: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "Cap 3 - variabili & mutability bt_06", 
    size: BUTTON_SIZE,    //larg + alt del bottone 
    position: (10, 350))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_06] )]
    button_06: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "Cap_04 - dangle - bt_07", 
    size: BUTTON_SIZE,    //larg + alt del bottone 
    position: (10, 400))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_07] )]
    button_07: nwg::Button,
    //---------------------------------------------------------------------------------------//

    //ATTENZIONE: deve essere rinominato il file main.rs a lib rs del progetto ownership che deve
    //essere incluso nel file cargo.toml del progetto generale inoltre nel file lib.rs occorre
    //rinominare la funzione principale main ---> run con tipo public: pub fn run() {...
    //---------------------------------------------------------------------------------------//
    //[nwg_control 	= creo un nuovo controllo con le proprieta text, size, position
    #[nwg_control(text: "Cap_04 - ownership - bt_08", 
	size: BUTTON_SIZE,    		//larg + alt del bottone 
	position: (10, 450))]
    //la posizione 10 X posizione + 50 Y
    //nwg_events  	= attivo l'evento del controllo con la funzione OnButtonClick
    //			la quale chiama la implementazione BasicApp e la funzione creata.
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_08] )]
    //creo il nuovo oggetto button con la libreria nwg
    button_08: nwg::Button,
    //---------------------------------------------------------------------------------------//
}

//le funzioni esterne
impl BasicApp {
    //buttone 1
    fn fn_button_esercizio_01(&self) {
       // call_exe::call("APRI_FILE_{guessing_game}.bat");
       //2 ° call con path
       call_exe::call("c:\\CASA\\PROGRAMMI\\RUST_PROGETTI\\APRI_FILE_{guessing_game}.bat");
    }

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
