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
extern crate branches;
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

const DIMENSIONI_WINDOWS: (i32, i32) = (800, 600);
const DIMENSIONI_CONTROL: (i32, i32) = (230, 35);

//01
/// FORM BASE IMPOSTAZIONI  @form.master
#[derive(Default)]
pub struct BasicAppCmb {
    // #[nwg_control(size: DIMENSIONI_WINDOWS, position: (300, 10), //cambio in ----> 300,10
    // title: "Rust Progetti con Combobox",
    // flags: "WINDOW|VISIBLE")] //evento messaggio 02
    window: nwg::Window,
    //---------------------------------------------------------------------------------------//

    // myflexbox: nwg::FlexboxLayout,

    // #[nwg_control(text: "Hello ",
    // size: (230, 35),                     //casella di testo largh + alt originale 280,35 ---- > 230,35
    // position: (10, 10), focus: true)]    //casella posizione x,y
    name_edit: nwg::TextInput,

    //@cmb.imposta.combobox,  @struttura.combobox, @struttura.cmb
    mycmb_01: nwg::ComboBox<&'static str>,             //@cmb_01.struct, @capitolo_01.cmb_01.struct
    mycmb_02: nwg::ComboBox<&'static str>,            //@cmb_02.struct  @capitolo_02.cmb_02.struct

    // TODO: per ogni capitolo, metti una combobox, che permette di scegliere gli esercizi
    //  mycmb2: nwg::ComboBox<'&static str>
}

//IMPLEMENTA COMBINATE 01,02... ECC..
impl BasicAppCmb {
    /// IMPL METODO CMB_01
    /// Imposta gli eventi a seconda dell'indice selezionato nella combo box
    /// # Arguments
    /// * `selezionato`: è l'indice della casella combinata selezionato. E' un Option di usize,
    /// intero senza segno  la cui dimensione usize la adatta a 32 o 64 bit a seconda del s.o.
    ///
    /// returns: ()
    fn fn_combo_box_cap_01_02_03(&self, selezionato: Option<usize>){
        //SELECT TRA INDICI SELEZIONATI DELLA COMBINATA
        match selezionato {
            //INDICE 0
            Some(0) => {
                //imposta la casella di testo : attenzione per ora non aggiorna subito risolvere con un thread
                self.name_edit.set_text("01_call_exe");
                call_exe::call(
                    "c:\\CASA\\PROGRAMMI\\RUST_PROGETTI\\Capitolo_01\\APRI_FILE_{HelloWord_eseguibile}.bat",
                );
            }
            //@definizione.metodo.CMB_01,  //@cmb_01.index,  @capitolo_01.cmb_01.index
            //INDICE 1
            Some(1) => {
                //imposta la casella di testo
                self.name_edit.set_text("02_guessing_game");
                guessing_game::game(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02
            //INDICE 2
            Some(2) => {
                //imposta la casella di testo
                self.name_edit.set_text("03_branches");
                branches::run();
            }
            //INDICE 3
            Some(3) => {
                //imposta la casella di testo
                self.name_edit.set_text("04_data_type");
                data_type::run();
            }
            //INDICE 4
            Some(4) => {
                //imposta la casella di testo
                self.name_edit.set_text("05_function");
                data_type::run();
            }
            //INDICE 5
            Some(5) => {
                //imposta la casella di testo
                self.name_edit.set_text("06_progetto_loop");
                data_type::run();
            }

            //INDICE 6
            Some(6) => {
                //imposta la casella di testo
                self.name_edit.set_text("07_variables_and_mutability");
                data_type::run();
            }

            //..... nuovo indice @cmb_01


            _ => {}  //INDICE NULL
        }
    }

    //FUNZIONE CMB_02
    fn fn_combo_box_cap_02(&self, selezionato: Option<usize>) {
        println!("evento della 2° cmb");
        // TODO: Per ogni capitolo implementa un metodo con un match per ogni combobox
        match selezionato {
            Some(_) => (),
            _ => ()
        }
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
        //attenzione ? = gli errorri vanno nel result al chiamante

        //TEXBOX = una per tutte le combinate , @texbox.builder
        nwg::TextInput::builder()
            .text("ESEMPIO COMBO BOX")
            .size((230, 35))
            .position((10, 10))
            .focus(true)
            .parent(&mut basic_app_cmb.window)
            .build(&mut basic_app_cmb.name_edit)?;

        //LISTA DI CMB CON VETTORE, @cmb_01.vec
        let vec_combo = vec![&mut basic_app_cmb.mycmb_01,
                             &mut basic_app_cmb.mycmb_02];
        // Todo aggiungi la nuova combobox al vettore come sotto, 3,4 idem...:
        //  vec![&mut basic_app_cmb.mycmb, &mut basic_app_cmb.mycmb2];

        //@vettori.di.vettori.combinate
        //VETTORE DI  comandi della cmb_01, @capitolo_01.cmb_01.vettore; @I.VETTORE.CMB
        //cmb_01.vec,
        let lista_di_vec: Vec<Vec<&str>> = vec![vec!["01_call_exe",
                                                     "02_guessing_game",
                                                     "03_branches",
                                                     "04_data_type" ,
                                                     "05_function" ,
                                                     "06_progetto_loop" ,
                                                     "07_variables_and_mutability" ],
                                                //@II.VETTORE.CMB,  @cmb_02.vec
                                                vec!["10", "20", "60"]];
        let mut i = 1;  //indice

        //CICLO FOR sul vettore di combobox per le combobox per adesso ce ne una,
        // + assegna la lista vettore indicizzata.  @cmb_01.builder
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
          move = sposta la proprieta delle variabili utilizzate
          dalla closure/lambda dentro la lambda */
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                //SELECT EVENTI DELLA FORM + FIGLI
                match evt {
                    nwg::Event::OnWindowClose => {
                        nwg::stop_thread_dispatch(); //termina il Thread l'applicazione altrimenti rimane attivo gestione attivita
                    }
                    //@CAPITOLO_01.CMB, @CMB.CAPITOLO_01
                    //qui sono gestite gli eventi DI TUTTE LE CMB CORRENTI + FUTURE
                    nwg::Event::OnComboxBoxSelection => {
                        //EVENTO CMB_01 FINITA, //@definizione.metodo.CMB_01.evento, @cmb_01.metodo,  @evento.cmb_01, @cmb_01.evento
                        if &handle == &ui.mycmb_01 { // controlla che l'handle da cui proviene l'evento sia quello della Combobox
                            // prendo l'indice dell'elemento selezionato della combobox
                            let selection = ui.mycmb_01.selection();
                            // Eseguo la funzione definita nella struct BasicAppCmb
                            ui.fn_combo_box_cap_01_02_03(selection);

                        //EVENTO CMB_02 DA FINIRE - //@definizione.metodo.CMB_02.evento, @evento.cmb_02
                        } else if &handle == &ui.mycmb_02 { // TODO: usa mycmb2
                            let selection = ui.mycmb_02.selection(); // TODO: usa mycmb2
                            ui.fn_combo_box_cap_02(selection); // DEFINIRE IL METODO DELLA NUOVA FUNZIONALITA !!!!!!
                        }


                        //TODO : aggiungere un else if per ogni combobox di un capitolo
                    }
                    _ => {} //NULLO
                }
            }
        }; //muoiono le variabili di evt_ui, definite fuori usate dentro

        //borrow_mut = prende a prestito ??? todo: da studiare??
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

    //VERSIONE NUOVA = modificata togliendo tutte le derive
    //inizializzo nwg + inizializzo i Font + attivo - @inizializzoBasicApp, @mainBasicApp
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    //build BasicAppCmb
    let _app = BasicAppCmb::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}







//region: COMBINATE - PROCEDIMENTO DI CREAZIONE -
    /*

 RIEPILOGO OPERAZIONI:
      01)IMPOSTA LA VARIABILE OGGETTO NELLA STRUTTURA BASIC APP DI WINDOWS
                                                @cmb_01.struct
      											@cmb_02.struct
      											@cmb_03.struct

      02)DEFINIZIONE IMPLEMENTAZIONE INDICE DELLE CASELLE COMBINATE
                                                @cmb_01.index
      											@cmb_02.index
      											@cmb_03.index

      03) IMPOSTAZIONE DELL'EVENTO DELLA COMBINATA 1, 2. ecc...
                                                @cmb_01.evento
      											@cmb_02.evento
      											@cmb_03.evento

      04) BUILDER, COSTRUZIONE DELLE COMBINATE, ASSOCIAZIONE DEL VETTORE, BUILDER DELLA WINDOWS E DELLA TEXT BOX UNICA
                                                @cmb_01.builder
                                                @Textbox_01.builder
                                                @cmb_01.vec

      ATTIVAZIONE MAIN
        viene attivata la struttura Basic App nella main:
                                                @inizializzoBasicApp, @mainBasicApp


    PROCEDIMENTO LA CREAZIONE DELLE  COMBINATE 1,2,..   @procedimento.creazione.combinate
    per impostare le combinate per capitoli occorre,
    01)	IMPOSTA LA VARIABILE OGGETTO NELLA STRUTTURA BASIC APP DI WINDOWS, all'interno della struttura pub struct BasicAppCmb {...
        che serve per creare una struttua windows mediante uma macro DERIVE, viene implementata all'interno
        di una WINDOWS BASE un elenco di combinate divise per capitolo.
        La Windows master avra come figli da 1 a 10 cmb per colonna.
        Per creare le combinate occorre definire diverse variabili oggetto mediante la macro nwg come ad es.
            mycmb: nwg::ComboBox<&'static str>, ec...
        la chiave di ricerca per la combinata per impostare la variabile oggetto  è la seguente: @cmb_01.struct


    02) DEFINIZIONE IMPLEMENTAZIONE INDICE DELLE CASELLE COMBINATE
        Le caselle combinate vengono gestite sia nella creazione dell'indice sia nella implementazione dello stesso nella
        funzione denominata
            fn fn_combo_box_cap_01_02_03(&self, selezionato: Option<usize>){...
        Questa funzione permette di creare ed impostare gli indici di scelta per ogni capitolo ed in questo caso essendo stata
        denominata ... _cap_01_02_03... la prima combinata deve gestire almeno i primi 3 capitolo di esercizi.
        L'indice viene gestito con le seguenti instruzioni:
             match selezionato {
                    //INDICE 0
                    Some(0) => {....
        l'indice della combinata parte da zero e si consiglia un massimo di 11 posizioni da 0-11.

        la chiave di ricerca per la combinata per impostare la variabile oggetto è la seguente: @cmb_01.index

    03) IMPOSTAZIONE DELL'EVENTO DELLA COMBINATA 1, 2. ecc...
        Per tutte le combinate create occorre impostare l'evento nella procedura
               match evt {....
        L'0ggetto evt gestisce tutti gli eventi della form Master e dei suoi figli e per quanto riguarda le combinate
        le stesse vengono gestite con la macro nwg
            nwg::Event::OnComboxBoxSelection => { ......
        questa macro permette di gestire gli eventi relativi solo ai figli della form e cioè le combinate che vengono
        individuate con la if di controllo tra gli Handle della windows e l'handle della combinata con la seguente istruzione
              if &handle == &ui.mycmb_01 {  ....
        Questa if confronta i due handle individua quella della combinata 01, recupera l'indice scelto e chiama la
        funzione di gestione degli indici individuata in
                 ui.fn_combo_box_cap_01_02_03(selection);
        Quindi per impostare la gestione degli eventi della combinata 1, 2 ecc... chiamare questa chiave:  @cmb_01.evento


    04) BUILDER, COSTRUZIONE DELLE COMBINATE, ASSOCIAZIONE DEL VETTORE, BUILDER DELLA WINDOWS E DELLA TEXT BOX UNICA
    	Nella funzione di implementazione della combinata
    		NativeUi<BasicAppCmbUi> for BasicAppCmb {
    	con il comando builder vengono implementate A) la costruzione della WINDOWS base, B)  la costruzione di una singola  TEXBOX da utilizzare
    	per tutte le combinate C) la costruzione con un ciclo for di tutte LE COMBINATE e con la contestuale assegnazione
    	della lista di VETTORI SCELTA per la COMBINATA 01








    */

//endregion: COMBINATE - PROCEDIMENTO DI CREAZIONE -