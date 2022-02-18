CREAZIONE DI UNA COMBINATA
Per creare una combinata occorre per prima cosa 
01) CREARE UNA STRUTTURA
02) IMPLMENTARE UNA STRUTTURA	
   02).01 creare una funzione oggetti che costruisca gli oggetti.... 	
struct BasicAppUi = creare una struttura



	LA IMPLEMENTAZIONE DELLA STRUTTURA
	impl nwg::NativeUi<BasicAppUi> for BasicApp {
		
		LA FUNZIONE BUILD
		fn build_ui(mut data_basic_pp: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {

				  //@cmb.crea = IMPOSTA LA COMBINATA
					nwg::ComboBox::builder()
					    .size((200, 300))
					    .collection(vec!["one", "two"])
					    .selected_index(Some(0))
					    .parent(&data_basic_pp.window)
					    .position((50, 50))
					    .build(&mut data_basic_pp.combo)?;





esempio 

//01_CARGO TOML = INSERIRE LE DIPENDENZE

[dependencies]

native-windows-gui={version="1.0.12",features =["listbox", "frame", "combobox", "flexbox"]}
native-windows-derive = "1.0.3" # Optional. Only if the derive macro is used.



//01 IMPOSTO LE VARIABILI OGGETTO
use native_windows_gui as nwg;
use super::*;           //SUPER ??
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;




02 CREO UNA STRUTTURA BASIC UP


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






#[derive(Default, NwgUi)]
/// LA SECONDA FORM IMPOSTAZIONI  @seconda.form
pub struct BasicAppUi {
    //inner = una struttura Rc a cui attribuire un parametro T che in questo caso è
    //la struttura stessa basic App.
    inner: Rc<BasicApp>,
    //variabile del handler della form base
    default_handler: RefCell<Option<nwg::EventHandler>>,
}
	
	impl nwg::NativeUi<BasicAppUi> for BasicApp {
		fn build_ui(mut data_basic_pp: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
			
			
		// Controls - 01 COSTRUISCO LA FORM BASE
		//CREA LA FINESTRA WINDOWS CON LE DIMENSIONI E LA POSIZIONE
		//---------------------------------------------------------------------------------------//
		//controllo base è questo size: (300, 135), position: (300, 300), per un bottone
		//la posizione base è questa position: (300, 300) cambio in ----> 300,10
		//paer avere la finestra centrale + la combinata
			
			//@form.master.crea
			nwg::Window::builder()
			.flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
			.size((600, 400))			//la grandezza	
			.position((300, 10))			//la  posizione
			//.flags ("WINDOW|VISIBLE")
			.title("FORM BASE CON LA CMB  example") //il titotolo della form
			.build(&mut data_basic_pp.window)?;
			
			
			//@cmb.crea = IMPOSTA LA COMBINATA
			        nwg::ComboBox::builder()
			            .size((200, 300))
			            .collection(vec!["one", "two"])
			            .selected_index(Some(0))
			            .parent(&data_basic_pp.window)
			            .position((50, 50))
			            .build(&mut data_basic_pp.combo)?;
		//---------------------------------------------------------------------------------------//


 




10 LA MAIN

fn main() {
    //SE FALLISCE LA COSTRUZIONE DELLA GUI MSG
    nwg::init().expect("Failed to init Native Windows GUI");
    //QUI METTE il tipo di testo ??
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
	
    //ATTENZIONE l'inserimento del codice presuppone una funzione associata in BasicApp.	
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
            
    //EVENTI
    nwg::dispatch_thread_events();
}
