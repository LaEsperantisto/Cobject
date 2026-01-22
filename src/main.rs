use cobject::cobject::CObject;
use cobject::ctitle_bar::CTitleBar;
use cobject::cwindow::CWindow;

fn main() {
    let mut window = CWindow::new(640, 480, "Test".into());

    let obj = CObject::new(100, 100, 100, 100, 0x0000FF00);
    let title = CTitleBar::new("Test".into(), &window);
    title.init(&mut window);

    while window.is_open() {
        window.poll_input();
        window.update();

        window.clear();
        window.draw(&obj);
        window.draw(&title);
        window.draw_window();
    }
}
