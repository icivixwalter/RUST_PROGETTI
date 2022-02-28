/* RUST PROGETTI CON MACRO DERIVE
    @progetto.macro.derive
    @derive.progetto.windows
    @windows.derive.progetto
    @finestra.semplice.con.derive
    @semplice.finestra.macro.derive
    @controlli.con.derive
    @cmb.derive.progetto
*/


extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use crate::seconda_gui;


const DIMENSIONI_WINDOWS: (i32, i32) = (800, 600);
const DIMENSIONI_CONTROL: (i32, i32) = (230, 35);

#[derive(Default, NwgUi)]   //@la.struttura.basicAppcmb
pub struct BasicAppCmb {

    //region:FORM BASE + TEXTBOX UNICA
    //*****************************************************************************//

    //LA  WINDOWS BASE
    #[nwg_control(
    size: DIMENSIONI_WINDOWS,
    position: (300, 10),
    title: "Rust Progetti con macro Derive",
    flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [BasicAppCmb::exit])] // necessario per uscire correttamente dall' app
    window: nwg::Window,

    //TEXBOX UNICA
    #[nwg_control(
    size: DIMENSIONI_CONTROL,
    position: (10, 10),
    text: "Scegli dalle choicebox",
    focus: true
    )]
    name_edit: nwg::TextInput,
    //*****************************************************************************//
    //endregion:FORM BASE + TEXTBOX UNICA

    //region:CMB_01 + LABEL_01 + EVENTI     CAPITOLO 1+2+3
    //*****************************************************************************//
    //LABEL DEI CONTROLLI COMBO BOX 1
    #[nwg_control(
    position: (10, 50),
    size: DIMENSIONI_CONTROL,
    text: "Capitoli 01, 02, 03")]
    mylbl_01: nwg::Label,

    //COMBO BOX 1 -
    #[nwg_control(collection: vec ! [
    "01_call_exe",
    "02_guessing_game",
    "03_branches",
    "04_data_type",
    "05_function",
    "06_progetto_loop",
    "07_variables_and_mutability"],
    position: (10, 80),
    size: DIMENSIONI_CONTROL,
    selected_index: Some(0))]
    #[nwg_events(OnComboxBoxSelection: [BasicAppCmb::cap_01_02_03])]  //EVENTI CMB_01
    mycmb_01: nwg::ComboBox<&'static str>,
    //*****************************************************************************//
    //endregion: CMB_01 + LABEL_01     CAPITOLO 1+2+3

    //region:CMB_02 + LABEL_02 + EVENTI      CAPITOLO 04
    //*****************************************************************************//

    //LABEL DEI CONTROLLI COMBO BOX 2
    #[nwg_control(
    position: (10, 120),
    size: DIMENSIONI_CONTROL,
    text: "Capitoli 04")]
    mylbl_02: nwg::Label,

    //COMBO BOX 2 - PER IL CAPITOLO 04
    #[nwg_control(collection: vec ! [
    "01_dangle",
    "02_ownership",
    "03_references_and_borrowing",
    "04_the_slice_type"

    ],
    position: (10, 150),
    size: DIMENSIONI_CONTROL,
    selected_index: Some(0))]
    #[nwg_events(OnComboxBoxSelection: [BasicAppCmb::cap_04])]      //EVENTI CMB_02 x il CAP 04
    mycmb_02: nwg::ComboBox<&'static str>,
    //*****************************************************************************//
    //endregion: CMB_02 + LABEL_02 + EVENTI      CAPITOLO 04

    //region:CMB_03 + LABEL_03 + EVENTI      CAPITOLO 05
    //*****************************************************************************//
    //@cmb_03, @capitolo.5.cmb.03, @label.03

    //LABEL DEI CONTROLLI COMBO BOX 3
    #[nwg_control(
    position: (10, 190),
    size: DIMENSIONI_CONTROL,
    text: "Capitoli 05")]
    mylbl_03: nwg::Label,

    //COMBO BOX 2 - PER IL CAPITOLO 05
    #[nwg_control(collection: vec ! [
    "01_can_old",
    "02_defining_methods",
    "03_format_debug",
    "04_multiple_impl_blocks",
    "05_operator_c",
    "06_ownership_2",
    "07_rettangolo",
    "08_struct_struttura"

    ],
    position: (10, 220),
    size: DIMENSIONI_CONTROL,
    selected_index: Some(0))]
    #[nwg_events(OnComboxBoxSelection: [BasicAppCmb::cap_05])]      //EVENTI CMB_03 x il CAP 05
    mycmb_03: nwg::ComboBox<&'static str>,
    //*****************************************************************************//
    //endregion: CMB_03 + LABEL_03 + EVENTI      CAPITOLO 05

    //region:cmb_04 + LABEL_05 + EVENTI      CAPITOLO_06
    //*****************************************************************************//
    //@cmb_04, @capitolo.06.cmb.05, @label.05 @mylbl_05  @06 @05 @260 @290

    //LABEL_05 DEI CONTROLLI COMBO_BOX_05
    #[nwg_control(
    position: (10, 260),
    size: DIMENSIONI_CONTROL,
    text: "Capitoli 06")]
    mylbl_05: nwg::Label,

    //COMBO_BOX_05 - PER IL CAPITOLO_06
    #[nwg_control(collection: vec ! [
    "01_bind_values",
    "02_catch_all",
    "03_config_max",
    "04_control_flow",
    "05_dice_roll",
    "06_dice_roll_2"

    ],
    position: (10, 290),
    size: DIMENSIONI_CONTROL,
    selected_index: Some(0))]
    #[nwg_events(OnComboxBoxSelection: [BasicAppCmb::cap_06])]      //EVENTI cmb_04 x il CAP_06
    mycmb_04: nwg::ComboBox<&'static str>,
    //*****************************************************************************//
    //endregion: cmb_04 + LABEL_05 + EVENTI      CAPITOLO_06

    //region:cmb_05 + LABEL_06 + EVENTI      CAPITOLO_07
    //*****************************************************************************//
    //@cmb_05, @capitolo.07.cmb.06, @label.06 @mylbl_06  @07 @06 @330 @370

    //LABEL_06 DEI CONTROLLI COMBO_BOX_06
    #[nwg_control(
    position: (10, 330),
    size: DIMENSIONI_CONTROL,
    text: "Capitolo 07")]
    mylbl_06: nwg::Label,

    //COMBO_BOX_06 - PER IL CAPITOLO_07
    #[nwg_control(collection: vec ! [
    "?01_bind_values_NON_ABILITATO",
    "?02_catch_all_NON ABILITATO",
    "03_restaurant"

    ],
    position: (10, 370),
    size: DIMENSIONI_CONTROL,
    selected_index: Some(0))]
    #[nwg_events(OnComboxBoxSelection: [BasicAppCmb::cap_07])]      //EVENTI cmb_05 x il CAP_07
    mycmb_05: nwg::ComboBox<&'static str>,
    //*****************************************************************************//
    //endregion: cmb_05 + LABEL_06 + EVENTI      CAPITOLO_07

}

impl BasicAppCmb {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    //region:************** CAPITOLO_07 CMB_05 ****************************
    fn cap_07(&self) {
        //SELECT TRA INDICI SELEZIONATI DELLA COMBINATA
        let selezionato = self.mycmb_05.selection().unwrap_or(0);
        match selezionato {
            //INDICE 0
            0 => {
                //imposta la casella di testo
                self.name_edit.set_text("?01_bind_values_NON_ABILITATO");
                bind_values::run(); // TODO: questo va spostato in fn_combo_box_cap_08()
            }            //@definizione.metodo.cmb_05

            //INDICE 1
            1 => {
                //imposta la casella di testo
                self.name_edit.set_text("?02_catch_all_NON ABILITATO");
                catch_all::run(); // TODO: questo va spostato in fn_combo_box_cap_08()
            }            //@definizione.metodo.cmb_05

            //INDICE 2 (indice 0: 2+1=3)
            2 => {
                //imposta la casella di testo
                self.name_edit.set_text("03_restaurant");
                restaurant::run(); // TODO: questo va spostato in fn_combo_box_cap_08()
            }            //@definizione.metodo.cmb_05



            _ => {}  //INDICE NULL
        }
    }
      //endregione:************** CAPITOLO_07 CMB_05 FINE************************


    //region:************** CAPITOLO 6 CMB_04 ****************************
    fn cap_06(&self) {
        //SELECT TRA INDICI SELEZIONATI DELLA COMBINATA
        let selezionato = self.mycmb_04.selection().unwrap_or(0);
        match selezionato {
            //INDICE 0
            0 => {
                //imposta la casella di testo
                self.name_edit.set_text("bind_values");
                bind_values::run(); // TODO: questo va spostato in fn_combo_box_cap_07()
            }            //@definizione.metodo.CMB_04

            //INDICE 1
            1 => {
                //imposta la casella di testo
                self.name_edit.set_text("02_catch_all");
                catch_all::run(); // TODO: questo va spostato in fn_combo_box_cap_07()
            }            //@definizione.metodo.CMB_04

            //INDICE 2 (indice 0: 2+1=3)
            2 => {
                //imposta la casella di testo
                self.name_edit.set_text("03_config_max");
                config_max::run(); // TODO: questo va spostato in fn_combo_box_cap_07()
            }            //@definizione.metodo.CMB_04

            //INDICE 3  (indice 0: 3+1=4)
            3 => {
                //imposta la casella di testo
                self.name_edit.set_text("04_control_flow");
                control_flow::run(); // TODO: questo va spostato in fn_combo_box_cap_07()
            }            //@definizione.metodo.CMB_04

            //INDICE 4  (indice 0: 4+1=5)
            4 => {
                //imposta la casella di testo
                self.name_edit.set_text("05_dice_roll");
                dice_roll::run(); // TODO: questo va spostato in fn_combo_box_cap_07()
            }            //@definizione.metodo.CMB_04

            //INDICE 5  (indice 0: 5+1=6)
            5 => {
                //imposta la casella di testo
                self.name_edit.set_text("06_dice_roll_2");
                dice_roll_2::run(); // TODO: questo va spostato in fn_combo_box_cap_07()
            }            //@definizione.metodo.CMB_04


            _ => {}  //INDICE NULL
        }
    }
    //endregion:************** CAPITOLO 6 CMB_04 FINE ********************


    //region:************** CAPITOLO 5 CMB_03 ****************************
    fn cap_05(&self) {
        //SELECT TRA INDICI SELEZIONATI DELLA COMBINATA
        let selezionato = self.mycmb_03.selection().unwrap_or(0);
        match selezionato {
            //INDICE 0
            0 => {
                //imposta la casella di testo
                self.name_edit.set_text("01_can_old");
                can_old::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02

            //INDICE 1
            1 => {
                //imposta la casella di testo
                self.name_edit.set_text("02_defining_methods");
                defining_methods::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02

            //INDICE 2 (indice 0: 2+1=3)
            2 => {
                //imposta la casella di testo
                self.name_edit.set_text("03_format_debug");
                format_debug::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02

            //INDICE 3  (indice 0: 3+1=4)
            3 => {
                //imposta la casella di testo
                self.name_edit.set_text("04_multiple_impl_blocks");
                multiple_impl_blocks::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02

            //INDICE 4  (indice 0: 4+1=5)
            4 => {
                //imposta la casella di testo
                self.name_edit.set_text("05_operator_c");
                operator_c::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02

            //INDICE 5  (indice 0: 5+1=6)
            5 => {
                //imposta la casella di testo
                self.name_edit.set_text("06_ownership_2");
                ownership_2::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02
            //INDICE 6  (indice 0: 6+1=7)
            6 => {
                //imposta la casella di testo
                self.name_edit.set_text("07_rettangolo");
                rettangolo::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02
            //INDICE 7  (indice 0: 7+1=8)
            7 => {
                //imposta la casella di testo
                self.name_edit.set_text("08_struct_struttura");
                struct_struttura::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02


            _ => {}  //INDICE NULL
        }
    }
    //endregion:************** CAPITOLO 5 CMB_03 FINE ********************


    //region:************** CAPITOLO 4 CMB_02 ****************************
    fn cap_04(&self) {
        //SELECT TRA INDICI SELEZIONATI DELLA COMBINATA
        let selezionato = self.mycmb_02.selection().unwrap_or(0);
        match selezionato {
            //INDICE 0
            0 => {
                //imposta la casella di testo
                self.name_edit.set_text("01_dangle");
                dangle::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02

            //INDICE 1
            1 => {
                //imposta la casella di testo
                self.name_edit.set_text("02_ownership");
                ownership::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02

            //INDICE 2 (indice 0: 2+1=3)
            2 => {
                //imposta la casella di testo
                self.name_edit.set_text("03_references_and_borrowing");
                references_and_borrowing::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02

            //INDICE 3  (indice 0: 3+1=4)
            3 => {
                //imposta la casella di testo
                self.name_edit.set_text("04_the_slice_type");
                the_slice_type::run(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02


            _ => {}  //INDICE NULL
        }
    }
    //endregion:************** CAPITOLO 4 CMB_02 fine ********************

    //region:************** CAPITOLO 1+2+3 CMB_01 ************************
    fn cap_01_02_03(&self) {
        //SELECT TRA INDICI SELEZIONATI DELLA COMBINATA
        let selezionato = self.mycmb_01.selection().unwrap_or(0);
        match selezionato {
            //INDICE 0
            0 => {
                //imposta la casella di testo : attenzione per ora non aggiorna subito risolvere con un thread
                self.name_edit.set_text("01_call_exe");
                call_exe::call(
                    "c:\\CASA\\PROGRAMMI\\RUST_PROGETTI\\Capitolo_01\\APRI_FILE_{HelloWord_eseguibile}.bat",
                );
            }
            //@definizione.metodo.CMB_01,  //@cmb_01.index,  @capitolo_01.cmb_01.index
            //INDICE 1
            1 => {
                //imposta la casella di testo
                self.name_edit.set_text("02_guessing_game");
                guessing_game::game(); // TODO: questo va spostato in fn_combo_box_cap_02()
            }            //@definizione.metodo.CMB_02
            //INDICE 2
            2 => {
                //imposta la casella di testo
                self.name_edit.set_text("03_branches");
                branches::run();
            }
            //INDICE 3
            3 => {
                //imposta la casella di testo
                self.name_edit.set_text("04_data_type");
                data_type::run();
            }
            //INDICE 4
            4 => {
                //imposta la casella di testo
                self.name_edit.set_text("05_function");
                function::run();
            }
            //INDICE 5
            5 => {
                //imposta la casella di testo
                self.name_edit.set_text("06_progetto_loop");
                progetto_loop::run();
            }

            //INDICE 6
            6 => {
                //imposta la casella di testo
                self.name_edit.set_text("07_variables_and_mutability");
                variables_and_mutability::run();
            }

            //..... nuovo indice @cmb_01

            _ => {}  //INDICE NULL
        }
    }
    //endregion:************** CAPITOLO 1+2+3 CMB_01 CMB_01 FINE *********
}
