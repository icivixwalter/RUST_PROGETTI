extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use crate::seconda_gui;


const DIMENSIONI_WINDOWS: (i32, i32) = (800, 600);
const DIMENSIONI_CONTROL: (i32, i32) = (230, 35);

#[derive(Default, NwgUi)]
pub struct BasicAppCmb {
    #[nwg_control(
    size: DIMENSIONI_WINDOWS,
    position: (300, 10),
    title: "Rust Progetti con macro Derive",
    flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicAppCmb::exit] )] // necessario per uscire correttamente dall' app
    window: nwg::Window,

    #[nwg_control(
    size: DIMENSIONI_CONTROL,
    position: (10,10),
    text: "Scegli dalle choicebox",
    focus: true
    )]
    name_edit: nwg::TextInput,

    #[nwg_control(
    position:(10,50),
    size: DIMENSIONI_CONTROL,
    text: "Capitoli 01, 02, 03")]
    mylbl_01: nwg::Label,

    #[nwg_control(collection: vec ! [
    "01_call_exe",
    "02_guessing_game",
    "03_branches",
    "04_data_type",
    "05_function",
    "06_progetto_loop",
    "07_variables_and_mutability"],
    position: (10,90),
    size: DIMENSIONI_CONTROL,
    selected_index: Some(0))]
    #[nwg_events(OnComboxBoxSelection: [BasicAppCmb::cap_01_02_03])]
    mycmb_01: nwg::ComboBox<&'static str>,
}

impl BasicAppCmb {
    fn exit(&self){
        nwg::stop_thread_dispatch();
    }

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
                data_type::run();
            }
            //INDICE 5
            5 => {
                //imposta la casella di testo
                self.name_edit.set_text("06_progetto_loop");
                data_type::run();
            }

            //INDICE 6
            6 => {
                //imposta la casella di testo
                self.name_edit.set_text("07_variables_and_mutability");
                data_type::run();
            }

            //..... nuovo indice @cmb_01

            _ => {}  //INDICE NULL
        }
    }
}
