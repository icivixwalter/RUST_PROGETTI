# esempi rust book - ita


# MODELLO DI CHIAMATA DELLA FUNZIONE

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







# LA FORM ESTERNA CREAZIONE ED APERTURA
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
        
   