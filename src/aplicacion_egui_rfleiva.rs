use native_dialog::{MessageDialog, MessageType};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct AplicacionDatosUsuario {
    label_dni: String,
    label_apellidos: String,
    label_nombre: String,
    label_sexo: String,
    label_nacionalidad: String,
    label_fecha: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value_edad: i32,
}

impl Default for AplicacionDatosUsuario {
    fn default() -> Self {
        Self {
            label_dni: "".to_owned(),
            label_apellidos: "".to_owned(),
            label_nombre: "".to_owned(),
            label_sexo: "".to_owned(),
            label_nacionalidad: "".to_owned(),
            label_fecha: "".to_owned(),
            value_edad: 0,
        }
    }
}

impl AplicacionDatosUsuario {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for AplicacionDatosUsuario {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label_dni,
            label_apellidos,
            label_nombre,
            label_sexo,
            label_nacionalidad,
            label_fecha,
            value_edad,
        } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Menú", |ui| {
                    if ui.button("Cerrar").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Tema", |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.vertical(|ui| {
                ui.label("");
                ui.label("Dni:");
                ui.text_edit_singleline(label_dni);
                ui.label("Apellidos:");
                ui.text_edit_singleline(label_apellidos);
                ui.label("Nombre:");
                ui.text_edit_singleline(label_nombre);
                ui.label("Sexo:");
                ui.text_edit_singleline(label_sexo);
                ui.label("Nacionalidad:");
                ui.text_edit_singleline(label_nacionalidad);
                ui.label("Fecha nacimiento:");
                ui.text_edit_singleline(label_fecha);
                ui.label("");
            });

            ui.add(egui::Slider::new(value_edad, 0..=100).text("Edad"));
            ui.label("");
            ui.horizontal(|ui| {
                ui.label("");
                if ui.button("Incrementar edad").clicked() {
                    *value_edad += 1;
                }
                ui.label("");
                ui.label("");
                if ui.button("Reducir edad").clicked() {
                    *value_edad -= 1;
                }
            });
            ui.label("");
            ui.horizontal(|ui| {
                ui.label("");
                ui.button("Guardar");
                ui.label("");
                if ui.button("Mostrar").clicked() {
                    let message = format!(
                        "Dni: {}\nApellidos: {}\nNombre: {}\nSexo: {}\nNacionalidad: {}\nFecha: {}\nEdad: {}",
                        label_dni, label_apellidos, label_nombre, label_sexo, label_nacionalidad, label_fecha ,value_edad
                    );
                    MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Datos introducidos")
                        .set_text(&message)
                        .show_alert()
                        .unwrap();
                }
                ui.label("");
                if ui.button("Limpiar").clicked() {
                    label_dni.clear();
                    label_apellidos.clear();
                    label_nombre.clear();
                    label_sexo.clear();
                    label_nacionalidad.clear();
                    label_fecha.clear();
                    *value_edad = 0;
                }
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
