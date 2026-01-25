use cobject::{ccolor, default_font, CImage, CText, CWindow};

fn main() {
    let mut window = CWindow::new(1000, 750, "Test".into());
    window.init();

    let mut text = CText::new(
        100,
        100,
        "ABCDEFGHIJKLMNOP123456".into(),
        default_font(),
        ccolor::GREEN,
    );
    text.scale(5, 5);
    let image = CImage::new(
        200,
        200,
        "/home/aster/dev/rust/Cobject/gfx/test.imace".into(),
    )
    .expect("Image not found");

    while window.is_open() {
        window.poll_input();
        window.update();

        window.fill(ccolor::BLACK);
        // window.draw(&text);
        window.draw(&text);
        window.draw(&image);
        window.show_window();
    }
}
