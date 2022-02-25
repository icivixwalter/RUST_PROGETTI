# esempi rust book - ita

# FLEX BOX - MODELLO DI PROGETTO - CREA UNA FLEX BOX DINAMYC
## COSTRUZIONE DI 4 BUTTON DINAMIC 
    //region: MODELLO PER COSTRUIRE UNA FLEX_BOX_DINAMIC DI BUTTON

    /*!
        MODELLO PER LA CREAZIONE DI 4 TEXT BOX DINAMICHE - MACRO DERIVE + MACRO NWG -
        Shows how to add controls dynamically into a flexbox layout
        faq: @modello.multibutton.derive
             @crea.multibutton.derive, @multi.box.dinamiche.derive
             @button.multipli.derive
             @creazione.4.textbox.dinamiche.derive
             @4button.dinamiche.derive, @4.button.dinamiche.derive, @04.multibox.dinamiche.derive, @4.multibox.dinamiche.derive
             @4.multibox.derive
    */
        extern crate native_windows_gui as nwg;
        extern crate native_windows_derive as nwd;
        use nwd::NwgUi;
        use nwg::{NativeUi, stretch};
        use stretch::geometry::Size;
        use stretch::style::*;

        use std::cell::RefCell;
        //CREAZIONE DELLA STRUTTURA FLEXBOX DINAMIC
        #[derive(Default, NwgUi)]
        pub struct FlexboxDynamic {         //INIZIO STRUTTURA FLEXBOXDINAMIC
            //CREA WINDOWS GUI MADRE
            #[nwg_control(size: (300, 500), position: (400, 200), title: "Flexbox example")]
            #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()], OnInit: [FlexboxDynamic::setup] )]
            window: nwg::Window,

            //OGGETTI FIGLI:
            //CREAZIONE DEL FLAXBOX LAYOUT
            #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
            layout: nwg::FlexboxLayout,

            //VARIABILE VETTORE BUTTON
            buttons: RefCell<Vec<nwg::Button>>,
        }
        impl FlexboxDynamic {
            //SETUP OGGETTO STESSO
            fn setup(&self) {
                //BORROW_MUT = prendi a prestito LA STRUTTURA BUTTONS
                let mut buttons = self.buttons.borrow_mut();
                //FOR PER CREARE 4 BUTTON
                for i in 0.. 4 {
                    //RENDE FIGLIO BUTTON DI WINDOWS??
                    buttons.push(nwg::Button::default());

                    let button_index = buttons.len() - 1;
                    //NWG = COSTRUISCE IL BUTTO
                    nwg::Button::builder()
                        .text(&format!("Button {}", i+1))
                        .parent(&self.window)
                        .build(&mut buttons[button_index]).expect("Failed to create button");

                    //STILE BUTTON  LARGHEZZA AUTOMATICA  @stile.button, @button.creazione.automatica
                    //                                    @button.auto.creato, @creazione.automatica.button
                    //                                    @modello.button.creazione.automatica

                    let style = Style {         //button.stile.automatico,  @stile.automaticao.button
                        size: Size { width: Dimension::Auto, height: Dimension::Points(100.0) },
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    };
                    //AGGIUNGE IL BUTTON AL CONTROLLO LAYOUT COME FIGLIO
                    self.layout.add_child(&buttons[button_index], style).expect("Failed to add button to layout");
                }
            }

        }
        fn main() {
            //INIZIALIZZO NWG + IMPOSTO IL FONT
            nwg::init().expect("Failed to init Native Windows GUI");
            nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
            //ATTIVO L'APPLICAZIONE
            let _app = FlexboxDynamic::build_ui(Default::default()).expect("Failed to build UI");

            nwg::dispatch_thread_events();
    }
    //endregion: MODELLO PER COSTRUIRE UNA FLEX_BOX_DINAMIC DI BUTTON


# FORM + TEXBOX - MODELLO PROGETTO DI UNA FORM + TEXBOX
## PROGETTO APP CON CMB + TXT - MACRO DERIVE

    /*
    PROGETTO DI PICCOLA APP NWG WINDOWS + TEXTBOX + BUTTON - MACRO DERIVE -
    con l'utilizzo delle derive, con tutti i punti  sotto elencati. Si considera piccola App con un
    numero di controlli < 10.
    Questo codice permette di costruire
    UNA FINESTRA PRINCIPALE DI WINDOWS  con 2 oggetti figli,
    UN COMBINATA
    UNA TEXBOX
    UN MSGBOX
    IL procedimento si articola in 8 punti:

    1. [solo se servono dati] Definisci una struttura che conterrà i dati dell'applicazione non GUI

    2. Definire tutte le strutture GUI che conterranno i controlli(Button, Window, ecc.) e dei dati (la struct al passo 1)

    3. [solo se servono dati] Aggiungi la struttura dei dati non-GUI ai membri della struttura GUI dentro un Refcell<>

    4. Definisci le funzioni delle GUI implementando le strutture della GUI

    5. Implementa NativeUi per la struttura della GUI (manualmente o con derive(NwGui))

    Nella funzione main():
    6. inizializzare la struttura dell'applicazione DATI NON GUI e la struttura della GUI
        1. Chiama build_ui con i dati inizializzati
        2. Avvia l'elaborazione degli eventi



    ATTENZIONE : NEL CARGO  TOM OCCORRE INSERIRE LE DIPENDENZE    native gui + native derive - NWG + NWD


    native-windows-gui={version="1.0.12",features =["listbox", "frame", "combobox", "flexbox"]}
    native-windows-derive = "1.0.3" # Optional. Only if the derive macro is used.

        faq: PROGETTO DI PICCOLA APP NWG WINDOWS + TEXTBOX + BUTTON - MACRO DERIVE -
        @progetto.costruisci.una.windows.derive
        @progetto.windows+form+cmb.derive
        @modello.form+textbox+cmb.derive
        @progetto.semplice.derive
        @progetto.macro.derive
        @derive.progetto.windows+cmb+textbox
        @macro.derive.progetto.windows+Cmb+TextBox

    */







    //**************************
    //   00 IMPOSTAZIONI
    //*****************************************************************************************
    //00_IMPOSTA I CRATE PER WINDOWS +  NWD + NWG + STD E LE VARIABILI GENERALI
    //--------------------------------------------------------------------------------//
    extern crate native_windows_gui as nwg;
    extern crate native_windows_derive as nwd;

        use nwd::NwgUi;
        use nwg::{NativeUi, Window, Button, TextInput};
        use std::cell::RefCell;
    
        const DIMENSIONI_WINDOWS: (i32, i32) = (800, 600);

	//--------------------------------------------------------------------------------//

    //*****************************************************************************************



    //**************************
    //   01 STRUTTURA DATI
    //*****************************************************************************************

            //01_CREA UNA STRUTTURA DATI WINDOWS
        
                #[derive(Default)] //Derive di rust non di nwg = quindi è MACRO DI RUST
                //libreria standard
                struct MyData {
                    my_nome: String,
                    my_eta: i32,
                }

    //*****************************************************************************************



    //**************************
    //   02 STRUTTURA GUI +
    //   03 AGGIUNGI DATI
    //*****************************************************************************************


            //02_DEFINISCI LE STRUTTURE DELLA GUI PADRE DEI CONTROLLI
            #[derive(Default, NwgUi)]                   //MACRO DERIVE PER CREARE UNA GUI DEFAULT
        
            //02_A_DEFINISCI LE STRUTTURE DELLA GUI PADRE DEI CONTROLLI OGGETTO + DEI LORO DATI  +
            //03 AGGIUGNI LA STRUTTURA DATI NON GUI AI MEMBRI DELLA STRUTTURA GUI
            pub struct MyApp {
        
            //03 AGGIUNGI LA STRUTTURA DATI AI MEMBRI DELLA STRUTTYRA MYAPP
                my_dati_non_gui: RefCell<MyData>,
        
            //MACRO NWG CREO WINDOWS +  EVENTI WINDOWS
                #[nwg_control(size: DIMENSIONI_WINDOWS, position: (300, 10), title: "Basic example", flags: "WINDOW|VISIBLE")]
                #[nwg_events(OnWindowClose: [MyApp::say_goodbye])] //EVENTO DI CHIUSURA DELL'APPLICAZIONE QUANDO SI PREME LA X
                window: Window,
                //---------------------------------------------------------------------------------------//
        
                // MACRO CASELLA DI TESTO - LABEL + SIZE + POSIZIONE - EDITABILE -
                //---------------------------------------------------------------------------------------//
                #[nwg_control(text: "Hello", size: (230, 35), position: (10, 10))]
                name_edit: TextInput,
                //---------------------------------------------------------------------------------------//
        
                //MACRO  BUTTON CON LA LABEL + POSIZIONE + EVENTO  CLICK
                #[nwg_control(text: "Cambia testo", size: (230, 35), position: (10, 50), focus: true)]
                #[nwg_events(OnButtonClick: [MyApp::text_init])]
                my_button: Button,
            }    

    //*****************************************************************************************




    //**************************
    //   04 IMPLEMENTO
    //   MY APP + I CONTROLLI
    //
    //*****************************************************************************************


        // 04_IMPLEMENTO MY APP
        // 4. Definisci le funzioni per la GUI MyApp in un blocco impl
        impl MyApp {
            //FUNZIONE USCITA PER MSG
            fn say_goodbye(&self) {
            nwg::modal_info_message(
                &self.window,
                "MESSAGGIO Goodbye",
                &format!("Goodbye {}", self.name_edit.text()),
            );
    
            //tread stop
            nwg::stop_thread_dispatch();
            }


        
            //FUNZIONE per INIZIALIZZARE IL TESTO DELLA TEXT INPUT
            fn text_init(&self) {
                //borrow = prendo in prestito l'oggeto MyData e lo salvo nella var. dati
                // restituendo un oggetto &MyData = Ref<MyData>
                let dati = self.my_dati_non_gui.borrow();
                //attenzione devi formattare dati
                let my_string = format!("nome: {}, eta: {}", dati.my_nome, dati.my_eta);
                self.name_edit.set_text(my_string.as_str());
        
        
                }
            }
        
        //*****************************************************************************************




        //**************************
        //   05 IMPLEMENTO
        //   NATIVE UI  ..... QUI E' SALTATO NON ESEGUITO ....
        //
        //*****************************************************************************************
        
        



        //**************************
        //   NELLA MAIN
        //   06 INIZIALIZZA STRUTTURA DATI NON GUI E STRUTTURA PRINCIPALE GUI
        //   07 CHIAMA BUILD_UI CON I DIANTI INIZIALIZZATI
        //   08 AVVIA L'ELABORAZIONE DEGLI EVENTI
        //*****************************************************************************************


        // ATTIVO LA MAIN E LE INIZIALIZZAZIONI
        fn main() {
        
            //INIZIALIZZO
            nwg::init().expect("Failed to init Native Windows GUI");
            
            // Opzionale: imposta il font
            nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
        
            // 6. Nella funzione main(), inizializzare la struttura DATI nonGUI e la struttura principale della GUI
            // creo l'oggetto di TIPO RefCell<MyData>  (tipo = RefCell<MyData>)
            let data: RefCell<MyData> = RefCell::new(MyData {
                my_eta: 51,
                my_nome: "walter".to_string(),
            });
            /// Istanzio l'app con i dati scritti sopra e i restanti valori di default
            let app: MyApp = MyApp {
                my_dati_non_gui: data,
                ..Default::default() // i campi rimanenti li inizializza Rust con i valori di default
            };
        
            // 7. Chiama build_ui con i dati inizializzati
            let _ui = MyApp::build_ui(app).expect("Failed to build UI");
        
            // 8. Avvia l'elaborazione degli eventi
            nwg::dispatch_thread_events();
        }


# FUNZIONE - MODELLO DI CHIAMATA DELLA FUNZIONE
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
La seconda operazione è creare l'oggetto button incrementale la cui funzione di attivazione rimane la stessa indicata sopra. ESEMPIO COMPLETO DI CREAZIONE DI UN OGGETTO BUTTON INCREMENTALE

    /*faq:  @bottone.incrementale, @come.costruire.u.bottone.incrementale,
            @procedura.completa.per.costruire.un.button.incrementale
            @procedura.completa.per.un.button.incrementale
            @chiamata.funzione, @creare.una.funzione.button, @funzione.evento.button
            @button.event
    
    */  
   //BUTTON 18) - II FILA DA 11 - 20 - PROGETTO ---> struct_struttura
  //---------------------------------------------------------------------------------------//
        #[nwg_control(text: "18) Cap_05 - struct_struttura", 
        //le dimensioni della Button è impostato in modo fissa dalla COSTANTE.
        size: FILA_02_BUTTON_SIZE,    //II° FILA DI 10 BOTTONI: larg + alt       del bottone 
        /*POSIZIONE DEL BUTTON è incrementale di 2,4,6,8,10 ecc.. partendo dalla posizione 100 
        si ha una automatica disposizione della casella text  INCREMENTANDO DI UN MULTIPLO
        DI 2 - Oggetto Button per il progetto - struct_struttura
        */
        position: (350, 100+(14*25)))]  //posizione incrementale dell'oggetto button, basta cambiare lo 0 con                            
        //un multiplo di due


        #[nwg_events( OnButtonClick: [BasicApp::fn_button_esercizio_18] )]
        button_18: nwg::Button,

  //---------------------------------------------------------------------------------------//


    //attivo la funzione con il parametro &self = this o se stesso
    fn fn_button_esercizio_18(&self) { //button_18 - progetto ---> struct_struttura
        struct_struttura::run(); 
    }
    
    #nel file toml
    struct_struttura={path="Capitolo_05/struct_struttura"} #button_18 - c:\CASA\PROGRAMMI\RUST_PROGETTI\Capitolo_05\struct_struttura\
    


       
### LA FUNZIONE DEL BUTTON INCREMENTALE
la funzione che viene chiamata dal bottone incrementale:

     //attivo la funzione con il parametro &self = this o se stesso
    fn fn_button_esercizio_15(&self) { //button_15 - progetto ---> operator_c
        operator_c::run(); 
    }

### FILE TOML PER IL BUTTON INCREMENTALE
nel file toml occorre impostare la seguente dicitura per il bottone incrementale

    operator_c={path="Capitolo_05/operator_c"} #button_15 - c:\CASA\PROGRAMMI\RUST_PROGETTI\Capitolo_05\operator_c\



# FORM ESTERNA - CREAZIONE E CHIAMATA ED  APERTURA
Come creare ed aprire una form esterna
faq: @aprire.una.form.esterna; @creare.una.form.esterna; @
     @creare.la.struttura.della.form.esterna; @chiama.la.form.esterna
     @la.form.esterna.come.si.crea; @form.esterna.creazione; @crea.la.form.esterna
## 01) CREA LA FORM ESTERNA
    Per creare una form esterna occorre per prima cosa :
        01) CREARE UNA STRUTTURA DELLA FORM
        02) CREARE UNA IMPLEMENTAZIONE
        03) ATTIVARE L'EVENTO DI APERTURA DELLA FORM

            
                    // region:LA_FORM_ESTERNA

            // LA FORM ESTERNA - STRUTTURA ED IMPLEMENTAZIONE
            //=======================================================================================//
            // 01) LE LIBRERIE ESTERNE : native windwos + nwg
            // 02) LE VARIABILI ISTANZA DELLE FORM E DEL TREAD
            // 03) LA MACRO DA CUI DERIVA LA FORM
            // 04) LA STRUTTURA DI COSTRUZIONE DELLA FORM
            // 05) LA IMPLEMENTAZIONE

            // region: Le_Variabili_Oggetto
            // 01) LE LIBRERIE ESTERNE : native windwos + nwg
            extern crate native_windows_gui as nwg;
            extern crate native_windows_derive as nwd;

            // 02) LE VARIABILI ISTANZA DELLE FORM E DEL TREAD
            use nwd::NwgUi;
            use nwg::NativeUi;
            use std::{thread, cell::RefCell};


            // 03) LA MACRO DA CUI DERIVA LA FORM
            #[derive(Default, NwgUi)]  //@II.FORM.04.Struttura.oggetto.form
            // endregion: Le_Variabili_Oggetto

            // region: struttura_YesNodDialog
            // 04) LA STRUTTURA DI COSTRUZIONE DELLA FORM
            pub struct YesNoDialog {
                data: RefCell<Option<String>>,

                #[nwg_control(size: (300, 115), position: (650, 300), title: "A dialog", flags: "WINDOW|VISIBLE")]
                #[nwg_events( OnWindowClose: [YesNoDialog::close] )]
                window: nwg::Window,

                #[nwg_control(text: "YES", position: (10, 10), size: (130, 95))]
                #[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
                choice_yes: nwg::Button,

                #[nwg_control(text: "NO", position: (160, 10), size: (130, 95), focus: true)]
                #[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
                choice_no: nwg::Button,
            }
            // endregion: struttura_YesNodDialog

            // region: Implementazione_YesNodDialog
            // 05) LA IMPLEMENTAZIONE
            //@II.FORM.05.Implementazione.oggetto.form
            impl YesNoDialog {
            
                /// Create the dialog UI on a new thread. The dialog result will be returned by the thread handle.
                /// To alert the main GUI that the dialog completed, this function takes a notice sender object.
                //TODO:NON E' USATA
                fn popup(sender: nwg::NoticeSender) -> thread::JoinHandle<String> {
                    thread::spawn(move || {
                        // Create the UI just like in the main function
                        let app = YesNoDialog::build_ui(Default::default()).expect("Failed to build UI");
                        nwg::dispatch_thread_events();
                        
                        // Notice the main thread that the dialog completed
                        sender.notice();

                        // Return the dialog data
                        app.data.take().unwrap_or("Cancelled!".to_owned())
                    })
                }

                fn close(&self) {
                    nwg::stop_thread_dispatch();
                }

                fn choose(&self, btn: &nwg::Button) {
                    let mut data = self.data.borrow_mut();
                    if btn == &self.choice_no {
                        *data = Some("No!".to_string());
                    } else if btn == &self.choice_yes {
                        *data = Some("Yes!".to_string());
                    }

                    self.window.close();
                }

            }
            // endregion: Implementazione_YesNodDialog

            //LA FORM ESTERNA - STRUTTURA ED IMPLEMENTAZIONE *** FINE ***
            //=======================================================================================//

            // endregion:LA_FORM_ESTERNA
## 02) ATTIVA LA FORM ESTERNA


            // region: attiva_la_form_esterna
            //BOTTONE 2 APRI SECONDA FORM - open_ii_form
            //---------------------------------------------------------------------------------------//
            /*APRO LA SECONDA FORM = mediante la creazione di un nuovo button  che attiva l'evento
                open_ii_form, questa FORM  contine gli ulteriori bottoni da 20 bottoni a 40 da creare. 
                    faq: @II.form, @apri.seconda.form, @crea.la.form.esterna; @apri.la.form.esterna 
                        @II.FORM.01.creo.BUTTON; @attiva.la.form.esterna; @chiama.la.form.esterna
            
            */
            #[nwg_control(text: "APRI LA II FORM", 
                        size: (230, 30),    //larg + alt del bottone da 280, 70 ---> ridotto 280, 30
                        position: (350, 50))]
            #[nwg_events( OnButtonClick: [BasicApp::open_ii_form] )]
            open_ii_form: nwg::Button, //BOTTONE 2 APRI SECONDA FORM - attiva l'evento Btn
            //---------------------------------------------------------------------------------------//
        // endregion: attiva_la_form_esterna
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
 
      
# HTLM PROGETTO DOC
    Il progetto viene ricreato in htlm con il comando cargo doc e si trova in questa directory
        il file htlm si chiama index
            @index,    @progetto.doc  @trova.index      
            posizione del file index
            C:\CASA\PROGRAMMI\RUST_PROGETTI\target\doc\progetto_loop 




# METODO STATICO - ESEMPIO DI STRUTTURA -
    struct Pippo {
        anni: i32,
        altezza: f32,
        nome_cane: String,
    }
    impl Pippo {
        fn new() -> Pippo {
            return Pippo{
                anni: 54,
                altezza: 5.6,
                nome_cane: "pluto".to_string()
            };
        }
   }
   fn main (){
     //usato il metodo statico per creare una istanza di Pippo e salvarla nella variabile 
        let pippo = Pippo::new();
   }
    