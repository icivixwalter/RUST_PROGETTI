/*!
A very simple application that show your name in a message box.
See `basic_d` for the derive version

PROGETTO CMB  con MODULO topolina implementato per lo studio dei moduli.

contiene una combinata semplice.

    @cmb.mod
    @cmb.cominata.con.modulo
    @combinata.con.modulo
    @cmb.mod


 */

extern crate native_windows_gui as nwg;

use nwg::NativeUi;

mod alternate; // importa il file alternate.rs

//STRUTTURA DEL FORM BASE SU CUI INNESTARE LA CMB *** FINE ***
//----------------------------------------------------------------------------//


pub fn main() {

    //ATTIVO IL MODULO TOPOLINA
    //-----------------------------------------------------------------------------//
    //per attivare il modulo occorre qualificare il modulo::funzione ---> variabile
    let a = topolina::Pippone {
        i: 10,
    };
    a.ciao();
    a.dove_sono();
    //-----------------------------------------------------------------------------//

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    //chiamo la form BASE
    let _ui = alternate::BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();

}//fine main

//creo la funzione del main
fn nome_modulo() -> String {
    return "main".to_string();
}


// COSTRUZIONE DI UN MODULO  - TOPOLINA
//===================================================================//
pub mod topolina {
    use crate::nome_modulo;

    //QUESTA E' UNA STRUTTURA
    pub struct Pippone {
        pub i: i32,
    }

    //Implementi la struct
    impl Pippone {

        //I° funzione ciao
        pub fn ciao(&self) {
            println!("utilizzo del Modulo - Sono Pippone {}", self.i);
        }
        // II° funzione
        pub fn dove_sono (&self) {
            let sono_qui = nome_modulo();
            println!("MOD: Sono Pippone {} sto nel modulo {}", self.i, sono_qui);

            let sono_qui = super::alternate::nome_modulo();
            println!("MOD: Sono Pippone {} ora sto nel modulo {}", self.i, sono_qui);
        }
    }
}
//===================================================================//