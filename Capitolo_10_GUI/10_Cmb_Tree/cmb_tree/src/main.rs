/*
    An application that load different interfaces using the partial feature.
    Partials can be used to split large GUI application into smaller bits.

    Requires the following features: `cargo run --example partials --features "listbox frame combobox flexbox"`
                                                                            tali opzioni possono essere attivate nel
                                                                            cargo --features ....
                                                                            o nel cargo.toml scrivendo questa
                                                                            dipendenza:
                                nwg={version="1.0.12",features =["listbox", "frame", "combobox", "flexbox"]}

    per utilizzare il nwg come in questo caso:
        extern crate native_windows_gui as nwg;
    e per evitare l'errore della mancanza di dipendenza  come questo:
             extern crate native_windows_gui as nwg;
    occorre inserire nel Toml almeno 1 delle due dipendenze per windows:

        FILE TOML.
            1)
                native-windows-gui = "1.0.12"
            2) opzionale
                native-windows-derive = "1.0.3" # Optional. Only if the derive macro is used.

                index
                c:/CASA/PROGRAMMI/RUST/cmb/cmb_01/target/doc/cmb_01/index.html


    TOML
    attenzione nel cargo tom occorre inserire queste dipendenze:

        [dependencies]
        #LA CASELLA COMBINATA COME PROGETTO SI TROVA QUI:
        # file:///C:/CASA/PROGRAMMI/RUST_PROGETTI/target/doc/native_windows_gui/struct.ComboBox.html#
        #native-windows-gui = "1.0.12"
        native-windows-gui={version="1.0.12",features =["listbox", "frame", "combobox", "flexbox"]}
        native-windows-derive = "1.0.3" # Optional. Only if the derive macro is used.

    faq:
        @cmb.tree
        @cmb.combinata.con.tree
        @tree+cmb  

*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;

//STRUTTURA + IMPLEMENTAZIONE = CLASSE PARTIAL DEMO
//=================================================================================//
#[derive(Default)]
pub struct PartialDemo {
    //attributi
    window: nwg::Window,

    layout: nwg::FlexboxLayout,       // vbox e hbox uninficate
    menu: nwg::ListBox<&'static str>, //menu = list box
    frame1: nwg::Frame,
    frame2: nwg::Frame,
    frame3: nwg::Frame,

    people_ui: PeopleUi,
    animal_ui: AnimalUi,
    food_ui: FoodUi,
}
//@01.inizio
//metodi della classe - PARTIAL DEMO CAMBIA LA SCENA A CON LA SELEZIONE DEL MENU
impl PartialDemo {
    //EVENTO @cambio.interfaccia
    //cambio intefaccia  @interfaccia.change
    fn change_interface(&self) {
        self.frame1.set_visible(false);
        self.frame2.set_visible(false);
        self.frame3.set_visible(false);

        let layout = &self.layout;
        if layout.has_child(&self.frame1) {
            layout.remove_child(&self.frame1);
        }
        if layout.has_child(&self.frame2) {
            layout.remove_child(&self.frame2);
        }
        if layout.has_child(&self.frame3) {
            layout.remove_child(&self.frame3);
        }

        use nwg::stretch::{
            geometry::Size,
            style::{Dimension as D, Style},
        };
        let mut style = Style::default();
        style.size = Size {
            width: D::Percent(1.0),
            height: D::Auto,
        };

        /*MENU 3 selezioni   @menu.select
            cambio la prospettiva solo se impostati a true
        */
        match self.menu.selection() {
            //NULLO | ZERO = FRAME 1
            None | Some(0) => {
                //@visualizza.3.layout = con True visualizza con selezione
                layout.add_child(&self.frame1, style).unwrap();
                self.frame1.set_visible(true); //true
            }
            // 1 = FRAME 2
            Some(1) => {
                layout.add_child(&self.frame2, style).unwrap();
                self.frame2.set_visible(true); //true
            }
            // 2 = FRAME 3
            Some(2) => {
                layout.add_child(&self.frame3, style).unwrap();
                self.frame3.set_visible(true); //true
                                               //self.frame3.set_visible(true);  //true
            }
            //> 3 = NESSUN FRAME
            Some(_) => unreachable!(),
        }
    }

    fn save(&self) {
        nwg::simple_message("Saved!", "Data saved!");
    }
    //@uscita.exit
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}
//=================================================================================//

#[derive(Default)]
pub struct PeopleUi {
    //@DUE.GRIGLIE DX + SX
    layout: nwg::GridLayout, //GridLayout PEOPLE
    layout2: nwg::GridLayout,

    label1: nwg::Label,
    label2: nwg::Label,
    label3: nwg::Label,

    name_input: nwg::TextInput,
    age_input: nwg::TextInput,
    job_input: nwg::TextInput,

    save_btn: nwg::Button,
}

/// ANIMALI crea la combo box
#[derive(Default)]
pub struct AnimalUi {
    //@DUE.GRIGLIE DX + SX
    layout: nwg::GridLayout, //GridLayout ANIMALI
    layout2: nwg::GridLayout,

    label1: nwg::Label,
    label2: nwg::Label,
    label3: nwg::Label,

    name_input: nwg::TextInput,
    race_input: nwg::ComboBox<&'static str>,
    is_soft_input: nwg::CheckBox,

    save_btn: nwg::Button,
}
/// FOOD
#[derive(Default)]
pub struct FoodUi {
    //@DUE.GRIGLIE DX + SX
    layout: nwg::GridLayout, //GridLayout FOOD
    layout2: nwg::GridLayout,

    label1: nwg::Label,
    label2: nwg::Label,

    name_input: nwg::TextInput,
    tasty_input: nwg::CheckBox,

    save_btn: nwg::Button,
}

//
// ALL of this stuff is handled by native-windows-derive
// TUTTA questa roba è gestita da native-windows-deriva
//
mod partial_demo_ui {
    use self::nwg::PartialUi;
    use super::*;
    use native_windows_gui as nwg;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    //struttura @PartialDemoUi
    pub struct PartialDemoUi {
        //@inner.01 == inserisce il data + handler default
        inner: PartialDemo,
        default_handler: RefCell<Vec<nwg::EventHandler>>,
    }

    /*IMPLEMENTA  NATIVE GUI
        il metodo
        NativeUI ----> come parametro una interfaccia UI
            Rc  = struttura con parametro Generico T
                       Rc -->
    */
    impl nwg::NativeUi<Rc<PartialDemoUi>> for PartialDemo {
        //funzione costruisci ui = @build_ui.function
        fn build_ui(mut data: PartialDemo) -> Result<Rc<PartialDemoUi>, nwg::NwgError> {
            use nwg::Event as E;

            // Controls LA FORM PRINCIPALE
            nwg::Window::builder()
                .size((500, 400))
                .position((300, 300))
                .title("CMB TREE UI - FORM MASTER") //@form.master.titolo
                .build(&mut data.window)?;
            //list box - combo box  @cmb.build
            nwg::ListBox::builder()
                .collection(vec!["People", "Animals", "Food"])
                .focus(true)
                .parent(&data.window)
                .build(&mut data.menu)?;

            // 3 frame 1,2,3
            //-------------------------------------------------------------------------//
            nwg::Frame::builder()
                .parent(&data.window)
                .build(&mut data.frame1)?; //frame 1

            nwg::Frame::builder()
                .flags(nwg::FrameFlags::BORDER) //frame 2
                .parent(&data.window)
                .build(&mut data.frame2)?;

            nwg::Frame::builder()
                .flags(nwg::FrameFlags::BORDER) //frame 3
                .parent(&data.window)
                .build(&mut data.frame3)?;
            //-------------------------------------------------------------------------//

            //@3.FRAME.BUILD
            // Partials costruisceo il frame 1 PEOPLE
            PeopleUi::build_partial(&mut data.people_ui, Some(&data.frame1))?;
            // costruisceo il frame 2 animali
            AnimalUi::build_partial(&mut data.animal_ui, Some(&data.frame2))?;
            // costruisceo il frame 3 FOOD
            FoodUi::build_partial(&mut data.food_ui, Some(&data.frame3))?;

            /* Wrap-up  - nuovo oggetto PartialDemoUI
                assegno alla variabile ui = user inteface
                il nuovo oggetto  PartialDemoUi
            */
            let ui = Rc::new(PartialDemoUi {
                //@inner.02 = inserisce il data + handler default
                inner: data,
                default_handler: Default::default(),
            });

            // @handle.windows = principale a cui appendi i 3 oggetti
            let mut window_handles = vec![&ui.window.handle];
            //APPEND sulla finestra le 3 ui
            window_handles.append(&mut ui.people_ui.handles());
            window_handles.append(&mut ui.animal_ui.handles());
            window_handles.append(&mut ui.food_ui.handles());

            for handle in window_handles.iter() {
                //variabile evento = clona ui
                let evt_ui = ui.clone();

                let handle_events = move |evt, evt_data, handle| {
                    evt_ui.people_ui.process_event(evt, &evt_data, handle);
                    evt_ui.animal_ui.process_event(evt, &evt_data, handle);
                    evt_ui.food_ui.process_event(evt, &evt_data, handle);

                    //SELECT CASE
                    match evt {
                        E::OnListBoxSelect => {
                            if &handle == &evt_ui.menu {
                                PartialDemo::change_interface(&evt_ui.inner);
                            }
                        }
                        E::OnWindowClose => {
                            if &handle == &evt_ui.window {
                                PartialDemo::exit(&evt_ui.inner);
                            }
                        }
                        E::OnButtonClick =>
                        //@button.salva
                        //se una delle tre le visualizzazioni click save allora salva
                        {
                            if &handle == &evt_ui.people_ui.save_btn
                                || &handle == &evt_ui.animal_ui.save_btn
                                || &handle == &evt_ui.food_ui.save_btn
                            {
                                PartialDemo::save(&evt_ui.inner);
                            }
                        }
                        _ => {} //none
                    }
                };

                ui.default_handler
                    .borrow_mut()
                    .push(nwg::full_bind_event_handler(handle, handle_events));
            }

            // Layout
            use nwg::stretch::{geometry::Size, style::Dimension as D};

            nwg::FlexboxLayout::builder()
                //costruisce la base TUTTO A SX
                .parent(&ui.window)
                .child(&ui.menu)
                .child_size(Size {
                    width: D::Percent(0.3),
                    height: D::Auto,
                })
                .child(&ui.frame1)
                .child_size(Size {
                    width: D::Percent(1.0),
                    height: D::Auto,
                })
                //AGGIUNGIU il nuovo layout che un Flexbox MEDIANTE QUESTO LO DISTRIBUISCE SULLO SCHERMO
                //distanziandolo.
                .build(&ui.layout)?;

            return Ok(ui);
        }
    }

    impl PartialDemoUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        /// Per assicurarsi che tutto venga liberato senza problemi, il gestore predefinito deve essere svincolato.
        pub fn destroy(&self) {
            //@ciclo.for = per cancellare tutti gli eventi ??
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
        }
    }

    impl Deref for PartialDemoUi {
        type Target = PartialDemo;

        fn deref(&self) -> &PartialDemo {
            &self.inner
        }
    }
}

//modulo @people.modulo
mod partial_people_ui {
    use self::nwg::{ControlHandle, NwgError, PartialUi};
    use super::*;
    use native_windows_gui as nwg;

    impl PartialUi for PeopleUi {
        fn build_partial<W: Into<ControlHandle>>(
            data: &mut PeopleUi,
            parent: Option<W>,
        ) -> Result<(), NwgError> {
            let parent = parent.unwrap().into();

            nwg::Label::builder()
                .text("Name:")
                .h_align(nwg::HTextAlign::Right)
                .parent(&parent)
                .build(&mut data.label1)?;

            nwg::Label::builder()
                .text("Age:")
                .h_align(nwg::HTextAlign::Right)
                .parent(&parent)
                .build(&mut data.label2)?;

            nwg::Label::builder()
                .text("Job:")
                .h_align(nwg::HTextAlign::Right)
                .parent(&parent)
                .build(&mut data.label3)?;

            nwg::TextInput::builder()
                .text("John Doe")
                .parent(&parent)
                .build(&mut data.name_input)?;

            nwg::TextInput::builder()
                .text("75")
                .flags(nwg::TextInputFlags::VISIBLE | nwg::TextInputFlags::NUMBER)
                .parent(&parent)
                .build(&mut data.age_input)?;

            nwg::TextInput::builder()
                .text("Programmer")
                .parent(&parent)
                .build(&mut data.job_input)?;

            nwg::Button::builder()
                .text("Save")
                .parent(&parent)
                .build(&mut data.save_btn)?;

            nwg::GridLayout::builder()
                .parent(&parent)
                .max_size([1000, 150])
                .min_size([100, 120])
                .child(0, 0, &data.label1)
                .child(0, 1, &data.label2)
                .child(0, 2, &data.label3)
                .child(1, 0, &data.name_input)
                .child(1, 1, &data.age_input)
                .child(1, 2, &data.job_input)
                .build(&data.layout)?; // 1° layout

            nwg::GridLayout::builder()
                //100,200
                .min_size([100, 200])
                .max_column(Some(2))
                .max_row(Some(6))
                .child(1, 5, &data.save_btn)
                .parent(&parent)
                .build(&data.layout2)?; // 2° layout

            Ok(())
        }

        fn process_event<'a>(
            &self,
            _evt: nwg::Event,
            _evt_data: &nwg::EventData,
            _handle: ControlHandle,
        ) {
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            Vec::new()
        }
    }
}

//modulo @Animal.modulo
mod partial_animal_ui {
    use self::nwg::{ControlHandle, NwgError, PartialUi};
    use super::*;
    use native_windows_gui as nwg;

    impl PartialUi for AnimalUi {
        //ciclo form per @Animal.for
        fn build_partial<W: Into<ControlHandle>>(
            data: &mut AnimalUi,
            parent: Option<W>,
        ) -> Result<(), NwgError> {
            let parent = parent.unwrap().into();

            nwg::Label::builder()
                .text("Name:")
                .h_align(nwg::HTextAlign::Right)
                .parent(&parent)
                .build(&mut data.label1)?;

            nwg::Label::builder()
                .text("Race:")
                .h_align(nwg::HTextAlign::Right)
                .parent(&parent)
                .build(&mut data.label2)?;

            nwg::Label::builder()
                .text("Is fluffy:")
                .h_align(nwg::HTextAlign::Right)
                .parent(&parent)
                .build(&mut data.label3)?;

            nwg::TextInput::builder()
                .text("Mittens")
                .parent(&parent)
                .build(&mut data.name_input)?;

            //@cmb.CREO
            nwg::ComboBox::builder()
                .collection(vec!["Cat", "Dog", "Pidgeon", "Monkey"])
                .selected_index(Some(0))
                .parent(&parent)
                .build(&mut data.race_input)?;

            nwg::CheckBox::builder()
                .text("")
                .check_state(nwg::CheckBoxState::Checked)
                .parent(&parent)
                .build(&mut data.is_soft_input)?;

            nwg::Button::builder()
                .text("Save")
                .parent(&parent)
                .build(&mut data.save_btn)?;

            nwg::GridLayout::builder()
                .parent(&parent)
                .max_size([1000, 150])
                .min_size([100, 120])
                .child(0, 0, &data.label1)
                .child(0, 1, &data.label2)
                .child(0, 2, &data.label3)
                .child(1, 0, &data.name_input)
                .child(1, 1, &data.race_input)
                .child(1, 2, &data.is_soft_input)
                .build(&data.layout)?;

            nwg::GridLayout::builder()
                .min_size([100, 200])
                .max_column(Some(2))
                .max_row(Some(6))
                .child(1, 5, &data.save_btn)
                .parent(&parent)
                .build(&data.layout2)?;

            Ok(())
        }

        fn process_event<'a>(
            &self,
            _evt: nwg::Event,
            _evt_data: &nwg::EventData,
            _handle: ControlHandle,
        ) {
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            Vec::new()
        }
    }
}

//modulo @food.modulo
mod partial_food_ui {
    use self::nwg::{ControlHandle, NwgError, PartialUi};
    use super::*;
    use native_windows_gui as nwg;

    impl PartialUi for FoodUi {
        fn build_partial<W: Into<ControlHandle>>(
            data: &mut FoodUi,
            parent: Option<W>,
        ) -> Result<(), NwgError> {
            let parent = parent.unwrap().into();

            nwg::Label::builder()
                .text("Name:")
                .h_align(nwg::HTextAlign::Right)
                .parent(&parent)
                .build(&mut data.label1)?;

            nwg::Label::builder()
                .text("Tasty:")
                .h_align(nwg::HTextAlign::Right)
                .parent(&parent)
                .build(&mut data.label2)?;

            nwg::TextInput::builder()
                .text("Banana")
                .parent(&parent)
                .build(&mut data.name_input)?;

            nwg::CheckBox::builder()
                .text("")
                .check_state(nwg::CheckBoxState::Checked)
                .parent(&parent)
                .build(&mut data.tasty_input)?;

            nwg::Button::builder()
                .text("Save")
                .parent(&parent)
                .build(&mut data.save_btn)?;

            nwg::GridLayout::builder()
                .parent(&parent)
                .max_size([1000, 90])
                .min_size([100, 80])
                .child(0, 0, &data.label1)
                .child(0, 1, &data.label2)
                .child(1, 0, &data.name_input)
                .child(1, 1, &data.tasty_input)
                .build(&data.layout)?;

            nwg::GridLayout::builder()
                .min_size([100, 200])
                .max_column(Some(2))
                .max_row(Some(6))
                .child(1, 5, &data.save_btn)
                .parent(&parent)
                .build(&data.layout2)?;

            Ok(())
        }

        fn process_event<'a>(
            &self,
            _evt: nwg::Event,
            _evt_data: &nwg::EventData,
            _handle: ControlHandle,
        ) {
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            Vec::new()
        }
    }
}

fn main() {
    println!("CMB TREE !");
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    //chiama costruisci @build_ui = user interface
    let ui = PartialDemo::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events(); // assegna gli eventi al thread.
    ui.destroy();
}
