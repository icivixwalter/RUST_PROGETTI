// region:LA_FORM_ESTERNA
// nome del file della form esterna: seconda_gui.rs

/// LA SECONDA FORM

/// LA FORM ESTERNA - STRUTTURA ED IMPLEMENTAZIONE -- @la.form.esterna, @seconda.form
///=======================================================================================//
/// 01) LE LIBRERIE ESTERNE : native windwos + nwg
/// 02) LE VARIABILI ISTANZA DELLE FORM E DEL TREAD
/// 03) LA MACRO DA CUI DERIVA LA FORM
/// 04) LA STRUTTURA DI COSTRUZIONE DELLA FORM
/// 05) LA IMPLEMENTAZIONE


// region: Le_Variabili_Oggetto
// 01) LE LIBRERIE ESTERNE : native windwos + nwg
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

// 02) LE VARIABILI ISTANZA DELLE FORM E DEL TREAD
use nwd::NwgUi;
// use nwg::NativeUi;
//use std::{thread, cell::RefCell};

//I° FILA DI 10 BOTTONI - DA 1-10
const FILA_01_BUTTON_SIZE: (i32, i32) = (310, 30); //larghezza + altezza fissi

//II° FILA DI 10 BOTTONI - DA 11-20 *** da utilizzare ***
// const FILA_02_BUTTON_SIZE: (i32, i32) = (310, 30); //larghezza + altezza fissi

//LA FINESTRA WINDOWS
const DIMENSIONI_WINDOWS: (i32, i32) = (800, 600); // (800,600)---> larghezza e posizione della finestra windows con 20 bottoni
                                                   //con 10 bottoni posizione e la grandezza della finestra windows era di (600, 635)
                                                   //con 2 bottoni posizione originale della finestra windows (300,135)

///@macro.deriva.la.form
/// 03) LA MACRO DA CUI DERIVA LA FORM
/*
@II.form.costruisci,  @la.II.form.struttura
region: struttura_YesNodDialog
04) LA STRUTTURA DI COSTRUZIONE DELLA FORM
*/
#[derive(Default, NwgUi)] //@II.FORM.04.Struttura.oggetto.form
                          // endregion: Le_Variabili_Oggetto

pub struct YesNoDialog {
    // data: RefCell<Option<String>>,

    //#[nwg_control(size: (300, 115), position: (650, 300), title: "A dialog", flags: "WINDOW|VISIBLE")]
    /// @CREA.FINESTRA.II
    /// 
    #[nwg_control(size: DIMENSIONI_WINDOWS, position: (150, 50), title: "A dialog", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [YesNoDialog::close] )]
    window: nwg::Window,

    //@button.yes
    #[nwg_control(text: "YES", position: (10, 10), size: (30, 35))]
    #[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
    choice_yes: nwg::Button,

    //@button.no
    #[nwg_control(text: "NO", position: (160, 10), size: (30, 35), focus: true)]
    #[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
    choice_no: nwg::Button,

    //@button_21.definizione
    #[nwg_control(text: "21) Cap 6 - BUTTON_21", 
    size: FILA_01_BUTTON_SIZE,    //III° FILA DI 10 BOTTONI: larg + alt       del bottone 
    position: (20, 100))]
    #[nwg_events( OnButtonClick: [YesNoDialog::fn_button_esercizio_21] )]
    button_21: nwg::Button,

    //@button_22.definizione
    #[nwg_control(text: "22) Cap_06 - BUTTON_22", 
    size: FILA_01_BUTTON_SIZE,    //III° FILA DI 10 BOTTONI: larg + alt       del bottone 
    position: (20, 140))]
    #[nwg_events( OnButtonClick: [YesNoDialog::fn_button_esercizio_22] )]
    button_22: nwg::Button,

    //@button_23.definizione
    #[nwg_control(text: "23) Cap_06 - BUTTON_23", 
    size: FILA_01_BUTTON_SIZE,    //III° FILA DI 10 BOTTONI: larg + alt       del bottone 
    position: (20, 180))]
    #[nwg_events( OnButtonClick: [YesNoDialog::fn_button_esercizio_23] )]
    button_23: nwg::Button,

    //@button_24.definizione
    #[nwg_control(text: "24) Cap_06 - BUTTON_24", 
    size: FILA_01_BUTTON_SIZE,    //III° FILA DI 10 BOTTONI: larg + alt       del bottone 
    position: (20, 220))]
    #[nwg_events( OnButtonClick: [YesNoDialog::fn_button_esercizio_24] )]
    button_24: nwg::Button,

    

    //---------------------------------------------------------------------------------------//
}
// endregion: struttura_YesNodDialog

// region: Implementazione_YesNodDialog
// 05) LA IMPLEMENTAZIONE
//@II.FORM.05.Implementazione.oggetto.form
impl YesNoDialog {
    // /// Create the dialog UI on a new thread. The dialog result will be returned by the thread handle.
    // /// To alert the main GUI that the dialog completed, this function takes a notice sender object.
    // //TODO:NON E' USATA
    // fn popup(sender: nwg::NoticeSender) -> thread::JoinHandle<String> {
    //     thread::spawn(move || {
    //         // Create the UI just like in the main function
    //         let app = YesNoDialog::build_ui(Default::default()).expect("Failed to build UI");
    //         nwg::dispatch_thread_events();

    //         // Notice the main thread that the dialog completed
    //         sender.notice();

    //         // Return the dialog data
    //         app.data.take().unwrap_or("Cancelled!".to_owned())
    //     })
    // }

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    fn choose(&self, btn: &nwg::Button) {
        // let mut data = self.data.borrow_mut();
        if btn == &self.choice_no {
            //    *data = Some("No!".to_string());
        } else if btn == &self.choice_yes {
            //    *data = Some("Yes!".to_string());
        }

        self.window.close();
    }

    //BUTTON_21
    //@button_21.funzione
    fn fn_button_esercizio_21(&self) {
    
        catch_all::run();

        nwg::modal_info_message(
            &self.window,
            "ATTIVATO EVENTO BUTTON",
            &format!("EVENTO DEL  BUTTON_21"),
        );
    }

    //BUTTON_22
    //@button_22.funzione
    fn fn_button_esercizio_22(&self) {
        //        variables_and_mutability::run(); //button_22

        dice_roll::run();

        nwg::modal_info_message(
            &self.window,
            "ATTIVATO EVENTO BUTTON",
            &format!("EVENTO DEL  BUTTON_22"),
        );
    }

    //BUTTON_23
    //@button_23.funzione
    fn fn_button_esercizio_23(&self) {
             //  dice_roll_2::run(); //button_23
            dice_roll_2::run();

            nwg::modal_info_message(
                &self.window,
                "ATTIVATO EVENTO BUTTON",
                &format!("EVENTO DEL  BUTTON_23"),
            );
        }

    
    //BUTTON_24
    //@button_24.funzione
    fn fn_button_esercizio_24(&self) {
        
        config_max::run();

        nwg::modal_info_message(
            &self.window,
            "ATTIVATO EVENTO BUTTON",
            &format!("EVENTO DEL  BUTTON_24"),
        );
    }

}
// endregion: Implementazione_YesNodDialog

//LA FORM ESTERNA - STRUTTURA ED IMPLEMENTAZIONE *** FINE ***
//=======================================================================================//

// endregion:LA_FORM_ESTERNA
