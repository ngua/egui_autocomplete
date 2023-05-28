#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
   // Log to stdout (if you run with `RUST_LOG=debug`).

   use egui_autocomplete_demo::TemplateApp;
   tracing_subscriber::fmt::init();

   let native_options = eframe::NativeOptions::default();
   eframe::run_native(
      "auto complete demo",
      native_options,
      Box::new(|_| Box::<TemplateApp>::default()),
   )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
   // Make sure panics are logged using `console.error`.

   use egui_autocomplete_demo::TemplateApp;
   console_error_panic_hook::set_once();

   // Redirect tracing to console.log and friends:
   tracing_wasm::set_as_global_default();

   let web_options = eframe::WebOptions::default();

   wasm_bindgen_futures::spawn_local(async {
      eframe::start_web(
         "the_canvas_id", // hardcode it
         web_options,
         Box::new(|_| Box::new(TemplateApp::default())),
      )
      .await
      .expect("failed to start eframe");
   });
}
