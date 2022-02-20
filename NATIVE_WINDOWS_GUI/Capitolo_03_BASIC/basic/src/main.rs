/*
3.01.01_Control builder
I controlli vengono creati utilizzando un'API del generatore. Su ogni tipo di controllo, il metodo può essere chiamato per creare un'istanza di un oggetto builder. I nomi dei costruttori utilizzano il seguente formato:‎ ‎builder [ControlName]Builder. Ex: (ButtonBuilder).

Ogni documentazione di controllo enumera le proprietà accettate dal generatore.

use native_windows_gui as nwg;
fn build_button(button: &mut nwg::Button, window: &nwg::Window, font: &nwg::Font) {
    nwg::Button::builder()
        .text("Hello")
        .flags(nwg::ButtonFlags::VISIBLE | nwg::ButtonFlags::CHECK)
        .font(Some(font))
        .parent(window)
        .build(button);
}

*/

// COSTRUZIONE DI UN BUTTON + EVENTO BUTTON
//===========================================================================//

use native_windows_gui as nwg;

//01_build= COSTRUICO IL BUTTON
//funzione costruisce il controllo button
fn build_button(button: &mut nwg::Button, window: &nwg::Window, font: &nwg::Font) {
    nwg::Button::builder() // -> ButtonBuilder 
        .text("Hello")
        .flags(nwg::ButtonFlags::VISIBLE | nwg::ButtonFlags::CHECK)
        .font(Some(font))
        .parent(window)
        .build(button);
}

//02_property= IMPOSTO LE PROPRIETA GET/SET
//funzione test button con impostazione delle proprieta Get/Set
fn test(button: &Button) {
    //imposto le proprieta del Button get/set
    button.visible(); // esempio di getter  [property]()
    button.set_visible(true); // esempio di setter  set_[property](value)
}

//===========================================================================//

// COSTRUZIONE DI UNA FORM + EVENTI FORM   
//===========================================================================//

//03_windows_builder = CREO LA FINESTRA BASE
fn crea_windows () {
nwg::Window::builder() //COSTRUISCO LA FINESTRA BASE
    .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
    .size((300, 115))
    .position((300, 300))
    .title("Basic example")
    .build(&mut data.window)?;
}

//3.02.01_Dispatching events
fn exit() {
    stop_thread_dispatch(); //blocco tread
}

//eventi thread 
fn _eventi_thread() {
    nwg::dispatch_thread_events();
    nwg::dispatch_thread_events_with_callback(|| {});
}

//TODO: da errori ??? sistemare
//CATTURARE L’EVENTO DELLA MASTER E DI TUTTI I SUOI FIGLI - 3.02.02_Hooking
//--------------------------------------------------------------------//
fn _evento() {
    //variabile UI – usa Rc
    let evt_ui = Rc::downgrade(&ui.inner);

    //variabile EVENTO = usa move |……..|
    let handle_events = move |evt, evt_data, handle| {
        //se esiste l’vento 	 della evt_ui.
        if let Some(events_ui) = evt_ui.upgrade() {
            //select CONTROLLO EVENTI 	DELLA MASTER E DI TUTTI I FIGLI
            match evt {
                //Form master = controllo evento close
                nwg::Event::OnWindowClose => {
                    if &handle == &events_ui.window {
                        //evento chiusura stopo thread
                        nwg::stop_thread_dispatch();
                    }
                }
                _ => {} //none figli = nessuno
            }
        }

        // FUNZIONE EVENTO full_bind_event_handler = aggancia l’evento
        // del CONTROLLO e del GENITORE
        nwg::full_bind_event_handler(handle, handle_events);

        //--------------------------------------------------------------------//
    };

    fn posizione_cursore(){
        //assegno alla variabile la posizione del cursore con il comando
        // GlobalCursor
        let (x,y) = nwg::GlobalCursor::position();  //non si crea una istanza ma viene utilizzato il comando rileva posizione mouse


    }

    //funzione copia appunti con &nwg::Window = parametro Hwnd di finestra
    // 3.03.04.01_Writing/Reading text  
    fn clipboard_text(window: &nwg::Window) {
        nwg::Clipboard::set_data_text(window, "Hello!");  //scrive negli appunti
        let text = nwg::Clipboard::data_text(window);     //?legge dagli appunti
        assert!(text.is_some());					//?appunti vuoti?		
        assert!(&text.unwrap() == &"Hello!");			//??
    }

}
// COSTRUZIONE DI UNA FORM + EVENTI FORM   *** FINE ***
//===========================================================================//




// I MESSAGGI
//===========================================================================//
//3.03.02_Message box
fn _messaggi() {
    let p = nwg::MessageParams {
        //accetta un parametro dalla finestra Padre che rimane 						     //diabilitata
        title: "Hey",
        content: "Cats are cute",
        buttons: nwg::MessageButtons::Ok,
        icons: nwg::MessageIcons::Warning,
    };
    assert!(nwg::message(&p) == nwg::MessageChoice::Ok)
}

//LE FUNZIONI MESSAGGIO:
pub fn fatal_message<'a>(title: &'a str, content: &'a str) {} //panico con messaggio di errore
pub fn error_message<'a>(title: &'a str, content: &'a str) {} // msg con icona di errore
pub fn simple_message<'a>(title: &'a str, content: &'a str) {} //msg con icona informativa

// I MESSAGGI *** FINE ***
//===========================================================================//




// PASSO LA STRUTTURA DEGLI APPUNTI    *** INIZIO ***
//===========================================================================//
/*È possibile passare la struttura dei dati utilizzando le funzioni degli 
appunti di basso livello. Si noti che tali funzioni non sono sicure perché
 non è possibile convalidare i dati

Quando si utilizzano le funzioni di basso livello, 
gli appunti devono essere aperti  manualmente e quindi CHIUSI. 
 Inoltre, quando si impostano i dati, spetta allo sviluppatore 
 svuotare qualsiasi contenuto fosse  negli appunti. 
 In caso contrario, si verificheranno errori sconosciuti.
 */



#[repr(C)]					//3.03.04.02_Handling custom data = Gestione dei dati personalizzati 
#[derive(Clone, Copy)]

//CREO UNA STRUTTURA HELLO
struct Hello {
    foo: usize,
    bar: [u16; 3]
}

//FUNZIONE SCRIVI NEGLI APPUNTI
fn write_custom_data(window: &nwg::Window) {
    //ASSEGNI ALLA VARIABILE LA STRUTTURA ED I VALORI	
    let data = Hello {
        foo: 6529,
        bar: [0, 100, 20]
    };
    //APRI GLI APPUNTI E LI SVUOTO		
    nwg::Clipboard::open(window);
    nwg::Clipboard::empty();

    unsafe {
        nwg::Clipboard::set_data(
            nwg::ClipboardFormat::Global("Hello"),
            &data as *const Hello,
            1
        );
    }
   //CHIUDO MANUALMENTE GLI APPUNTI ALTRIMENTI ERRORI SCONOSCIUTI
    nwg::Clipboard::close();
}
    
//LEGGI I DATI
fn read_custom_data(window: &nwg::Window) -> Option<Hello> {
    unsafe {
        nwg::Clipboard::open(window); //APRI GLI APPUNTI
        let data = nwg::Clipboard::data(nwg::ClipboardFormat::Global("Hello"));
        nwg::Clipboard::close();  //CHIUDI GLI APPUNTI
        data
    }
}
//LEGGO GLI APPUNTI ??
fn read_custom_data_handle(window: &nwg::Window) -> Option<Hello> {
    unsafe {
        nwg::Clipboard::open(window);
        let handle = nwg::Clipboard::data_handle(nwg::ClipboardFormat::Global("Hello"));
       //CONFRONTO DATA CON HANDLE
        let data = match handle {
            Some(h) => {  //SE ESISTE VALORE
                let data_ptr: *const Hello = h.cast();  //??
                let data = *data_ptr;
                h.release(); 
                Some(data)
            },
            None => None //SELECT NONE
        };
      //CHUSURA DEGLI APPUNTI
        nwg::Clipboard::close();
        data
    }
}

// PASSO LA STRUTTURA DEGLI APPUNTI    *** FINE ***
//===========================================================================//




fn main() {
    println!("CAPITOLO 3 BASIC come costruire un controllo button !");

    //per ora non collego nullo
}


