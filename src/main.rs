// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::{
    egui::{
        self,
        ViewportBuilder,
        Rect,
        FontId,
        FontFamily,
        Label,
        RichText,
        Image,
    },
    HardwareAcceleration,
    Renderer
};
use std:: {
    thread,
    sync:: {
        Arc,
        Mutex,
    },
    time::{
        Duration,
        Instant,
    },
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        hardware_acceleration: HardwareAcceleration::Required,
        renderer: Renderer::Glow,
        vsync: false,
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "FlynnyViz",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::new(cc))
        }),
    )
}

struct MyApp {
    open_file_picker: bool,
    comms: Arc<Mutex<String>>,
    debug: Debug,
}

pub struct Debug {
    enabled: bool,
    continuous_render: bool,
    framerate: u16,
    start_frame: Instant,
    end_frame: Instant,
    widget_font: FontId,

}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let open_file_picker = false;
        let comms = Arc::new(Mutex::new(String::from("")));
        let debug = Debug {
            enabled: false,
            continuous_render: false,
            start_frame: Instant::now(),
            end_frame: Instant::now(),
            framerate: 0,
            widget_font: FontId::new(12.0, FontFamily::Monospace),
            
        };
        let slf = Self {
            debug,
            open_file_picker,
            comms,
        };
        slf
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Framerate calculations
            self.debug.end_frame = Instant::now();
            self.debug.framerate = (1000000000 / self.debug.end_frame
                .checked_duration_since(self.debug.start_frame)
                .unwrap_or(Duration::from_secs(0))
                .as_nanos()) as u16;
            self.debug.start_frame = Instant::now();

            // Debug corner
            egui::SidePanel::right("Debug panel").show(ctx, |ui| {
                // ui.label("Hello World!");
                ui.add(egui::Checkbox::new(&mut self.debug.enabled, "Enable debug info"));
                if self.debug.enabled == true {
                    // FPS
                    ui.add(Label::new(RichText::new(String::from(format!("FPS {:?}", self.debug.framerate))).monospace().font(self.debug.widget_font.clone())));
                    // Continuous render
                    ui.add(egui::Checkbox::new(&mut self.debug.continuous_render, RichText::new(String::from("Enable continuous rendering")).font(self.debug.widget_font.clone())));
                    // debug text size
                    // ui.add(egui::Slider::new(&mut self.debug.widget_font.size, 1.0..=48.0).text("Widget size"));

                    // ui.put(widget_rect, Label::new(RichText::new(String::from(format!("FPS\t{:?}", self.framerate))).monospace().font(self.widget_font.clone())));
                }
             });

            // File selection
            // TODO: Fix formats other than .png not working
            if ui.button("Open file...").clicked() {
                self.open_file_picker = true;
            }
            if self.open_file_picker == true {
                let comms = self.comms.clone();
                thread::spawn(move || {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("images (.jpeg, .jpg, .gif, .png, .webp)", &["jpeg", "jpg", "png", "gif", "webp"])
                        .pick_file(){
                            match comms.lock() {
                                Ok (mut lock) => {
                                    *lock = path.display().to_string()

                                }
                                Err (error) => {
                                    println!("Error aquiring lock to comms: \n{error}")
                                }
                            }
                        }
                });
                self.open_file_picker = false;
            }
            match self.comms.try_lock() {
                Ok(string) => {
                    ui.add(
                        Image::new(format!("file://{string}"))
                    )
                },
                Err(_err) => {todo!();}
            };
            if self.debug.continuous_render {ctx.request_repaint();}
        });
    }
}


// fn align_to_top_right(ctx: &egui::Context, widget_size: (f32, f32)) -> Rect {
//     let screen_size = ctx.screen_rect();
//     let widget_width = widget_size.0;
//     let widget_height = widget_size.1;

//     let top_right_x = screen_size.max.x - widget_width;
//     let top_right_y = screen_size.min.y;

//     let top_right = egui::Pos2::new(top_right_x, top_right_y);

//     Rect::from_min_size(top_right, egui::Vec2::new(widget_width, widget_height))
// }
