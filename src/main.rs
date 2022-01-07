/*!
    A very simple application that show your name in a message box.
    See `basic` for the version without the derive macro
*/

extern crate guessing_game;
extern crate data_type;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
// use std::{sync::Mutex, thread};
// use std::sync::atomic::AtomicBool;
// static spawned: Mutex<bool> = Mutex::new(false);

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
    size: (280, 30),    //larg + alt del bottone 
    position: (10, 100))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_01] )]
    button_01: nwg::Button,
    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "Cap 2 - Guessing Game", 
        size: (280, 30),    //larg + alt del bottone 
        position: (10, 150))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_02] )]
    button_02: nwg::Button,
    //---------------------------------------------------------------------------------------//
    #[nwg_control(text: "Cap 3 - Data type", 
        size: (280, 30),    //larg + alt del bottone 
        position: (10, 200))]
    #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_03] )]
    button_03: nwg::Button,
    //---------------------------------------------------------------------------------------//
}

//le funzioni esterne
impl BasicApp {
    //buttone 1
    fn fn_button_esercizio_01(&self) {
        call_exe::call("APRI_FILE_{guessing_game}.bat");
    }

    //buttone 2
    fn fn_button_esercizio_02(&self) {
        guessing_game::game(); // TODO spawna un thread che esegue game()
    }


    fn fn_button_esercizio_03(&self) {
        data_type::run(); 
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
