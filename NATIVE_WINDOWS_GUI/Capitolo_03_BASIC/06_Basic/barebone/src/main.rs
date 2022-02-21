/**
    A very simple application that show your name in a message box.
    This demo shows how to use NWG without the NativeUi trait boilerplate.
    Note that this way of doing things is alot less extensible and cannot make use of native windows derive.
    See `basic` for the NativeUi version and `basic_d` for the derive version
*/
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
use nwd::NwgUi;



use std::rc::Rc;

fn main() {

    //inizializzazione
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    //attenzione DOVE SONO DEFINITE 2 STRUTTURE 1 I DATI DELL'APPLICAZIONE NON GUI
    //LA SECONDA STRUCT I DATI DELLA GUI ??
    let mut window = Default::default();
    let mut name_edit = Default::default();
    let mut hello_button = Default::default();
    let layout = Default::default();

    //finestra
    nwg::Window::builder()
        .size((300, 115))
        .position((300, 300))
        .title("Basic example")
        .build(&mut window)
        .unwrap();
    //casella di testo
    nwg::TextInput::builder()
        .text("Heisenberg")
        .focus(true)
        .parent(&window)
        .build(&mut name_edit)
        .unwrap();
    //button
    nwg::Button::builder()
        .text("Say my name")
        .parent(&window)
        .build(&mut hello_button)
        .unwrap();

    //grid layout
    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child(0, 0, &name_edit)
        .child_item(nwg::GridLayoutItem::new(&hello_button, 0, 1, 1, 2))
        .build(&layout)
        .unwrap();


    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(
        &window.handle, move |evt, _evt_data, handle| 
        {
        use nwg::Event as E;    //E variabile oggetto eventi

        //EVENTI DELLA FORM MASTER    
        match evt {
            E::OnWindowClose => //evento on close della finestra
                if &handle == &events_window as &nwg::Window {
                    //MSG finestra di messaggio
                    nwg::modal_info_message(
                        &events_window.handle, 
                        "Goodbye", 
                        &format!("Goodbye {}", name_edit.text()));
                    //stop thread della finestra master
                    nwg::stop_thread_dispatch();
                },
            //evento del click button
            E::OnButtonClick => 
                //confronto tra handle 
                if &handle == &hello_button {
                    //msg MESSAGGIO
                    nwg::modal_info_message(
                    &events_window.handle, "Hello", 
                    &format!("Hello {}", name_edit.text()));
                },
            _ => {} //NONE
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}








// fn main() {
//     println!("Hello, world!");
// }
