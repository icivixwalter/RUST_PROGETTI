/*
PROGETTO APP NWG
con l'utilizzo delle derive, con tutti i punti  sotto elencati.

    NO 1. [solo se servono dati] Definisci una struttura che conterrà i dati dell'applicazione non GUI

    2. Definire tutte le strutture GUI che conterranno i controlli(Button, Window, ecc.) e dei dati (la struct al passo 1)

    NO 3. [solo se servono dati] Aggiungi la struttura dei dati non-GUI ai membri della struttura GUI dentro un Refcell<>

    4. Definisci le funzioni delle GUI implementando le strutture della GUI

    5. Implementa NativeUi per la struttura della GUI (manualmente o con derive(NwGui))

Nella funzione main():
    6. inizializzare la struttura dell'applicazione e la struttura della GUI
    7. Chiama build_ui con i dati inizializzati
    8. Avvia l'elaborazione degli eventi


   COSTRUISCO UNA FORM BASE con una

*/

extern crate data_type;
extern crate guessing_game;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

mod seconda_gui;
mod main2;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
// use nwd::NwgUi;
use nwg::{NativeUi, NwgError, WindowFlags};
use nwg::stretch::geometry::Rect;
use nwg::stretch::style::FlexDirection;

const DIMENSIONI_WINDOWS: (i32, i32) = (800, 600);
const DIMENSIONI_CONTROL: (i32, i32) = (230, 35);


/// LA SECONDA FORM IMPOSTAZIONI  @seconda.form
#[derive(Default)]
pub struct BasicAppCmb {
    // #[nwg_control(size: DIMENSIONI_WINDOWS, position: (300, 10), //cambio in ----> 300,10
    // title: "Rust Progetti con Combobox",
    // flags: "WINDOW|VISIBLE")] //evento messaggio 02
    window: nwg::Window,
    //---------------------------------------------------------------------------------------//

    // myflexbox: nwg::FlexboxLayout,

    // #[nwg_control(text: "Hello ",
    // size: (230, 35),                    //casella di testo largh + alt originale 280,35 ---- > 230,35
    // position: (10, 10), focus: true)] //casella posizione x,y
    name_edit: nwg::TextInput,

    mycmb: nwg::ComboBox<&'static str>,
    mycmb2: nwg::ComboBox<&'static str>,

    // TODO: per ogni capitolo, metti una combobox, che permette di scegliere gli esercizi
    //  mycmb2: nwg::ComboBox<'&static str>
}

//IMPLEMENTA COMBINATA
impl BasicAppCmb {
    fn fn_combo_box_cap_01(&self, selezionato: Option<usize>) {
        //SELECT TRA INDICI SELEZIONATI
        match selezionato {
            //INDICE 0
            Some(0) => {
                //imposta la casella di testo : attenzione per ora non aggiorna subito risolvere con un thread
                self.name_edit.set_text("call_exe");
                call_exe::call(
                    "c:\\CASA\\PROGRAMMI\\RUST_PROGETTI\\Capitolo_01\\APRI_FILE_{HelloWord_eseguibile}.bat",
                );
            }
            //INDICE 1
            Some(1) => {
                //imposta la casella di testo
                self.name_edit.set_text("guessing_game");
                guessing_game::game(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }
            _ => {}  //INDICE NULL
        }
    }

    fn fn_combo_box_cap_02(&self, selezionato: Option<usize>) {
        println!("evento della 2° cmb");
        // TODO: Per ogni capitolo implementa un metodo con un match per ogni combobox
        // match selezionato {
        //     Some(_) => (),
        //     _ => ()
        // }
    }
}

//STRUTTURA BasicAppUi  con i dati di BasicApp + variabile handler
pub struct BasicAppCmbUi {
    //inner = una struttura Rc a cui attribuire un parametro T che in questo caso è
    //la struttura stessa basic App.
    inner: Rc<BasicAppCmb>,
    //variabile del handler della form base
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl NativeUi<BasicAppCmbUi> for BasicAppCmb {
    fn build_ui(mut basic_app_cmb: Self) -> Result<BasicAppCmbUi, NwgError> {
        nwg::Window::builder()
            .size(DIMENSIONI_WINDOWS)
            .position((300, 10))
            .title("Rust Progetti con Combobox")
            .flags(WindowFlags::VISIBLE | WindowFlags::WINDOW)
            .build(&mut basic_app_cmb.window)?;


        // nwg::FlexboxLayout::builder()
        //     .parent(&mut basic_app_cmb.window)
        //     .flex_direction(FlexDirection::Column)
        //     .auto_spacing(Some(10))
        //     .build(&mut basic_app_cmb.myflexbox)?;
        //attenezione ? = gli errorri vanno nel result al chiamante

        nwg::TextInput::builder()
            .text("ESEMPIO COMBO BOX")
            .size((230, 35))
            .position((10, 10))
            .focus(true)
            .parent(&mut basic_app_cmb.window)
            .build(&mut basic_app_cmb.name_edit)?;

        //LISTA DI CMB CON VETTORE
        let vec_combo = vec![&mut basic_app_cmb.mycmb,
                             &mut basic_app_cmb.mycmb2];
        // Todo aggiungi la nuova combobox al vettore come sotto, 3,4 idem...:
        //  vec![&mut basic_app_cmb.mycmb, &mut basic_app_cmb.mycmb2];

        //VETTORE DI  comandi della cmb_01
        let lista_di_vec: Vec<Vec<&str>> = vec![vec!["ESERCIZIO_01", "ESERCIZIO_02"],
                                                vec!["10", "20", "60"]];
        let mut i = 1;

        //CICLO FOR sul vettore di combobox per le combobox per adesso ce ne una,
        // + assegna la lista vettore indicizzata.
        for combo in vec_combo {
            //
            let index: Vec<&str> = lista_di_vec[i - 1].clone();

            nwg::ComboBox::builder()
                .parent(&mut basic_app_cmb.window)
                .position((10, 60 * i as i32))
                .size(DIMENSIONI_CONTROL)
                .collection(index)
                .selected_index(None)
                .build(combo)?;
            i += 1;
        }


        let ui = BasicAppCmbUi {
            //nuovo oggetto inner con i dati di dataBasicApp
            inner: Rc::new(basic_app_cmb),
            //variabile evento dell'oggetto.
            default_handler: Default::default(),
        };


        /* EVENTI DI TUTTI GLI OGGETTI WINDOWS + FIGLI*/
//-----------------------------------------------------------------------------//
        let evt_ui = Rc::downgrade(&ui.inner);
        /*LAMBDA = closure rust = funzione senza NOME,
          move = sopsta la proprieta delle variabili utilizzate
          dalla closure/lambda dentro la lambda */
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                //SELECT EVENTI DELLA FORM + FIGLI
                match evt {
                    nwg::Event::OnWindowClose => {
                        nwg::stop_thread_dispatch(); //termina il Thread l'applicazione altrimenti rimane attivo gestione attivita
                    }
                    //qui sono gestite gli eventi DI TUTTE LE CMB CORRENTI + FUTURE
                    nwg::Event::OnComboxBoxSelection => {
                        //EVENTO CMB_01 FINITA
                        if &handle == &ui.mycmb { // controlla che l'handle da cui proviene l'evento sia quello della Combobox
                            // prendo l'indice dell'elemento selezionato della combobox
                            let selection = ui.mycmb.selection();
                            // Eseguo la funzione definita nella struct BasicAppCmb
                            ui.fn_combo_box_cap_01(selection);
                            //EVENTO CMB_02 DA FINIRE
                        } else if &handle == &ui.mycmb2 { // TODO: usa mycmb2
                            let selection = ui.mycmb2.selection(); // TODO: usa mycmb2
                            ui.fn_combo_box_cap_02(selection); // DEFINIRE IL MEDOTO DELLA NUOVA FUNZIONALITA !!!!!!
                        }
                        //TODO : aggiungere un else if per ogni combobox di un capitolo
                    }
                    _ => {} //NULLO
                }
            }
        }; //muoiono le variabili di evt_ui, definite fuori usate dentro

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));
//-----------------------------------------------------------------------------//
        //return
        Ok(ui)
    }
}

//CANCELLO LA BASE APP UI
impl Drop for BasicAppCmbUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

//IMPLEMENTO LA BASE APP UI
impl Deref for BasicAppCmbUi {
    type Target = BasicAppCmb;

    fn deref(&self) -> &BasicAppCmb {
        &self.inner
    }
}

fn main() {
    //VERSIONE VECCHIA = manin2
    //richiamo il progetto originale
    // main2::main2();

    //VERSIONE NUOVA = mani modificata togliendo tutte le derive

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicAppCmb::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}