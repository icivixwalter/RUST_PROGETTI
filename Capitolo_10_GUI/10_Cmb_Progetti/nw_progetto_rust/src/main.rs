/*
        FUNZIONAMENTO GENERALE
Dovrebbe essere su tre livelli per utilizzare le API DI WINDOWS. Per ottenere questo si pensa
che ci sono 3 strati così esposti:

1° LIVELLO
--------------------------------------------
    IL PROGRAMMA RST  e le sue
    implementazioni

2° LIVELLO
--------------------------------------------
rust Utiliza  la LIBRERIA DI RUST  +
                 LE API DI RUST
 che dovrebbero attivare le api di windows


 3° LIVELLO
--------------------------------------------
API DI WINDOWS  a cui doverbbero attingere le
api di rust per recuperare un oggetto es.:
    con borrow_mut ... dovrebbe prendere recuperare l'oggetto windows??
    *basic_app_ui.default_handler.borrow_mut() ..... ?=?
        ??? studiare se è cosi.!!!
            ........



ATTENZIONE ALLE DIPENDENZE PER IL CARGO TOML

.... DA INSERIRE
[dependencies]

native-windows-gui={version="1.0.12",features =["listbox", "frame", "combobox", "flexbox"]}
native-windows-derive = "1.0.3" # Optional. Only if the derive macro is used.

    faq:
    @cmb.semplice
    @combinata.semplice
    @form.cmb.semplice

 */


//01 IMPOSTO LE VARIABILI OGGETTO
use native_windows_gui as nwg;
use native_windows_gui::NativeUi;

//use super::*;           //SUPER ?? SE NON VIENE INSERITO IN UN FILE SPERATO
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;


//STRUTTURA DEL FORM BASE SU CUI INNESTARE LA CMB
//=======================================================================================//
//  NOTE: STRUTTURA 01 - BASICAPP
#[derive(Default)]
pub struct BasicApp {
    //NELLA STRUTTURA BASIC APP imposto la form master + cmb SOLO COME VARIABILI DELLA STRUTTURA
    window: nwg::Window,
    //@cmb.imposta la variabile
    combo: nwg::ComboBox<&'static str>,

    button: nwg::Button,
}

//Implemento la struttura 01 Basic Up
impl BasicApp {
    // nella implementazione RICHIAMO UN MSG DI USCITA
    fn say_goodbye(&self) {
        //msg di uscita
        nwg::simple_message("CHIUSURA FINESTRA: Goodbye", "Goodbye Walter");
        //fine tread
        nwg::stop_thread_dispatch();
    }
}

//=======================================================================================//


// II STRUTTURA
//=======================================================================================//

//STRUTTURA BasicAppUi  con i dati di BasicApp + variabile handler
pub struct BasicAppUi {
    //inner = una struttura Rc a cui attribuire un parametro T che in questo caso è
    //la struttura stessa basic App.
    inner: Rc<BasicApp>,
    //variabile del handler della form base
    default_handler: RefCell<Option<nwg::EventHandler>>,
}


//BASE APP UI = IMPLEMENTA NWG+TRAIT nativeUi(UI)+ STRUTTURA BASE
//NativeUi = Trait - intefaccia
impl nwg::NativeUi<BasicAppUi> for BasicApp {
    /*FUNZIONE BUILD
    imposto la funzione di costruzione inserita nella  main  e questa
    funzione costruisce gli oggetti FORM + CMB  con gli eventi con 1 parametro:
        dataBasicApp = struttura  BasicApp

    */
    fn build_ui(mut data_basic_pp: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
        //EVENTI
        use nwg::Event as E;

        // Controls - 01 COSTRUISCO LA FORM BASE
        nwg::Window::builder()
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .size((600, 400))
            .position((300, 300))
            .title("FORM NUOVO PROGETTO RUST CON LA COMBINATA SEMPLICE -")
            .build(&mut data_basic_pp.window)?;

        nwg::Button::builder()
            .text("Hello")
            .flags(nwg::ButtonFlags::VISIBLE)
            .parent(&data_basic_pp.window)
            .build(&mut data_basic_pp.button);

        //@cmb.crea = IMPOSTA LA COMBINATA
        nwg::ComboBox::builder()
            .size((200, 300))
            .collection(vec!["one", "two"])
            .selected_index(Some(0))
            .parent(&data_basic_pp.window)
            .position((50, 50))
            .build(&mut data_basic_pp.combo)?;

        // Wrap-up assegno a ui un nuovo oggetto  struttura  BasicAppUi
        let basic_app_ui = BasicAppUi {
            //nuovo oggetto inner con i dati di dataBasicApp
            inner: Rc::new(data_basic_pp),
            //variabile evento dell'oggetto.
            default_handler: Default::default(),
        };

        // Events - GESTISCO GLI EVENTI
        let evt_ui = Rc::downgrade(&basic_app_ui.inner);

        // Lambda/CLOSURE = funzione anonima |parametri| {...codice...}
        let handle_events = move |evt, _evt_data, handle| {

            //IF LET = rileva Option e se esiste il valore lo assegna ad 'ui', altrimenti salta l'if
            if let Some(ui) = evt_ui.upgrade() {
                //SELECT EVENTO WINDOW
                match evt {
                    E::OnWindowClose =>
                        if &handle == &ui.window {
                            BasicApp::say_goodbye(&ui);
                        },
                    _ => {} //NULLO
                }
            }
        };
         // *ui = derefenzio il puntatore al fine di aggiornare il valore puntato con il Some(...)
         *basic_app_ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
             &basic_app_ui.window.handle, handle_events)
         );

        return Ok(basic_app_ui);
    }
} //fine implementazione struttura

/// Drop = Trait - interfaccia implementata per
/// BasicAppUi: CODICE CHIAMATO PRIMA DI DISTRUGGERE L'OGGETTO
impl Drop for BasicAppUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    /// Per assicurarsi che ogni cosa è liberata dalla memoria senza problemi.
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

//IMPLEMENTO LA BASE APP UI
impl Deref for BasicAppUi {
    type Target = BasicApp;

    fn deref(&self) -> &BasicApp {
        &self.inner
    }
}


//=======================================================================================//


fn main() {
    println!("Hello, COMBINATA SEMPLICE!");

    //SE FALLISCE LA COSTRUZIONE DELLA GUI MSG
    nwg::init().expect("Failed to init Native Windows GUI");
    //QUI METTE il tipo di testo ??
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");


    //EVENTI
    nwg::dispatch_thread_events();
}
