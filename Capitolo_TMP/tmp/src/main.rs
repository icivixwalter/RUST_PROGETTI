
//region: MODELLO PER COSTRUIRE UNA FLEX_BOX_DINAMIC DI BUTTON

    /*!
        MODELLO PER LA CREAZIONE DI 4 TEXT BOX DINAMICHE - MACRO DERIVE + MACRO NWG -
        Shows how to add controls dynamically into a flexbox layout
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