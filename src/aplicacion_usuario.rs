extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use native_windows_gui::MessageChoice;
use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (400, 200), position: (400, 200), title: "Aplicación datos usuario", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::cerrar_aplicacion] )]
    window: nwg::Window,

    #[nwg_control(text: "Nombre:", size: (70, 25), position: (10, 10))]
    name_label: nwg::Label,
    #[nwg_control(text: "", size: (280, 25), position: (100, 10))]
    nombre_edit: nwg::TextInput,
    #[nwg_control(text: "Apellidos:", size: (70, 25), position: (10, 40))]
    apellidos_label: nwg::Label,
    #[nwg_control(text: "", size: (280, 25), position: (100, 40))]
    apellidos_edit: nwg::TextInput,
    #[nwg_control(text: "Fecha:", size: (70, 25), position: (10, 70))]
    fecha_label: nwg::Label,
    #[nwg_control(text: "", size: (280, 25), position: (100, 70))]
    fecha_edit: nwg::TextInput,
    #[nwg_control(text: "DNI:", size: (70, 25), position: (10, 100))]
    dni_label: nwg::Label,
    #[nwg_control(text: "", size: (280, 25), position: (100, 100))]
    dni_edit: nwg::TextInput,

    #[nwg_control(text: "Guardar datos", size: (100, 30), position: (25, 140))]
    #[nwg_events( OnButtonClick: [BasicApp::guardar_datos] )]
    boton_guardar_datos: nwg::Button,
    #[nwg_control(text: "Mostrar datos", size: (100, 30), position: (145, 140))]
    #[nwg_events( OnButtonClick: [BasicApp::mostrar_datos] )]
    boton_mostrar_datos: nwg::Button,
    #[nwg_control(text: "Limpiar", size: (100, 30), position: (265, 140))]
    #[nwg_events( OnButtonClick: [BasicApp::limpiar_datos] )]
    boton_limpiar_datos: nwg::Button,
}

impl BasicApp {
    fn mostrar_datos(&self) {
        nwg::simple_message(
            "Datos usuario",
            &format!(
                "\t Nombre: {}\t
                 Apellidos: {}\t
                 Fecha: {}\t
                 DNI: {}",
                self.nombre_edit.text(),
                self.apellidos_edit.text(),
                self.fecha_edit.text(),
                self.dni_edit.text(),
            ),
        );
    }

    fn guardar_datos(&self) -> MessageChoice {
        let nombre = &self.nombre_edit.text();
        let apellidos = &self.apellidos_edit.text();
        let fecha = &self.fecha_edit.text();
        let dni = &self.dni_edit.text();

        let data = format!(
            "Nombre: {}\nApellidos: {}\nFecha: {}\nDNI: {}\n",
            nombre, apellidos, fecha, dni
        );

        match std::fs::write("C:\\Users\\rfleiva\\Desktop\\data.txt", data) {
            Ok(_) => nwg::simple_message(
                "Datos guardados",
                "Los datos han sido guardados en el archivo data.txt",
            ),
            Err(error) => {
                nwg::simple_message("Error", &format!("Error al guardar los datos: {}", error))
            }
        }
    }

    fn limpiar_datos(&self) {
        self.nombre_edit.set_text("");
        self.apellidos_edit.set_text("");
        self.fecha_edit.set_text("");
        self.dni_edit.set_text("");
    }

    fn cerrar_aplicacion(&self) {
        nwg::simple_message("Cierre", &format!("Cerrando aplicación"));
        nwg::stop_thread_dispatch();
    }
}

pub fn aplicacion_usuario() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
