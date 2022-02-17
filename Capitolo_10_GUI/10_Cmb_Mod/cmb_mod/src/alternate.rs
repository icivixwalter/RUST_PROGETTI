/*
// ALL of this stuff is handled by native-windows-derive
// QUESTA E' LA FORM CON LA COMBINATA
01
Struct
    BasicAppUi = costruisco una nuova struttura assegnandola alla variabile
                inner, recuperando la struttura BasicApp della main dove sono
                state definite 2 variabili base, per la form e la cmb.
02
Impl
    nwg::NativeUi<BasicAppUi>
                = creo la implementazione della struttura con al suo interno la copia
                della BasicAppUi che contiene BasicApp e la assegno
                alla variabile
                    inner: Rc<BasicApp>,
                02.01
                builder
                    02.01.01
                    Nella implementazione
                        creo la variabile eventi  E

                    02.01.02
                        costruisco la form + la cmb




*/

use native_windows_gui as nwg;
// use super::BasicApp;           //SUPER ??
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;


//STRUTTURA DEL FORM BASE SU CUI INNESTARE LA CMB
//----------------------------------------------------------------------------//
#[derive(Default)]
pub struct BasicApp {
    //NELLA STRUTTURA BASIC APP imposto la form master + cmb SOLO COME VARIABILI DELLA STRUTTURA
    window: nwg::Window,
    //@cmb.imposta la variabile
    combo: nwg::ComboBox<&'static str>,
}

impl BasicApp {
    // nella implementazione RICHIAMO UN MSG DI USCITA
    fn say_goodbye(&self) {
        nwg::simple_message("Goodbye", "Goodbye Walter");
        nwg::stop_thread_dispatch();
    }
}


//STRUTTURA BasicAppUi  con i dati di BasicApp + variabile handler
pub struct BasicAppUi {
    //inner = una struttura Rc a cui attribuire un parametro T che in questo caso è
    //la struttura stessa basic App.
    inner: Rc<BasicApp>,
    //variabile del handler della form base
    default_handler: RefCell<Option<nwg::EventHandler>>,
}
//BASE APP UI = IMPLEMENTA NWG+TRAIT nativeUi(UI)+ STRUTTURA BASE
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
            .title("FORM CMB MODE CON MODULO + CMB + funzioni CIAO   example")
            .build(&mut data_basic_pp.window)?;
        //@cmb.crea = IMPOSTA LA COMBINATA
        nwg::ComboBox::builder()
            .size((200, 300))
            .collection(vec!["one", "two"])
            .selected_index(Some(0))
            .parent(&data_basic_pp.window)
            .position((50, 50))
            .build(&mut data_basic_pp.combo)?;

        // Wrap-up assegno a ui un nuovo oggetto  struttura  BasicAppUi
        let ui = BasicAppUi {
            //nuovo oggetto inner con i dati di dataBasicApp
            inner: Rc::new(data_basic_pp),
            //variabile evento dell'oggetto.
            default_handler: Default::default(),
        };

        // Events - GESTISCO GLI EVENTI
        let evt_ui = Rc::downgrade(&ui.inner);

        //VARIABILE EVENTI ??
        let handle_events = move |evt, _evt_data, handle| {

            if let Some(ui) = evt_ui.upgrade() {
                //SELECT EVENTO
                match evt {
                    E::OnWindowClose =>
                        if &handle == &ui.window {
                            BasicApp::say_goodbye(&ui);
                        },
                    _ => {} //NULLO
                }
            }
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

        return Ok(ui);
    }
} //fine implementazione struttura

//CANCELLO LA BASE APP UI
impl Drop for BasicAppUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
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


//creo la funzione per alternate
pub fn nome_modulo() -> String {
    return "alternate".to_string();
}
