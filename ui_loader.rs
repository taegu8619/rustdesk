use sciter::Window;

pub fn load_ui() {
    const INDEX_HTML: &[u8] = include_bytes!("../src/ui/index.html");

    let mut frame = Window::new();
    if frame.load_html(INDEX_HTML, Some("this://app/")) {
        frame.run_app();
    } else {
        eprintln!("Failed to load embedded UI.");
    }
}
