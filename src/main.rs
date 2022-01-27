//
// native:
// cargo build --release
// wasm:
// cargo build --release --lib --target wasm32-unknown-unknown
//
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = qr2text::DemoApp::default();
    eframe::run_native(Box::new(app), Default::default());
}
