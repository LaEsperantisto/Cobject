use cobject::{
    ccolor, default_font, get_platform, CBox, CBoxCannon, CImage, CPlayer, CText, CWindow,
};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut window = CWindow::new(1700, 900, "Test".into());
    window.init();

    let text = Rc::new(RefCell::new(CText::new(
        0,
        100,
        "IM KAY:!".into(),
        default_font(),
        ccolor::RED,
    )));
    text.borrow_mut().scale(4, 4);
    window.add_drawable(text);
    let image = CImage::new(
        200.0,
        200.0,
        "/home/aster/dev/rust/Cobject/gfx/test.imace".into(),
    )
    .expect("Image not found");

    let player = Rc::new(RefCell::new(CPlayer::new(100.0, 400.0)));

    window.add_input_listener(player.clone());
    window.add_object(player.clone());
    let my_box = Rc::new(RefCell::new(CBoxCannon::new(200.0, 1.0, 100, 100)));
    window.add_object(my_box.clone());
    window.add_input_listener(my_box.clone());
    let my_box = Rc::new(RefCell::new(CBox::new(500.0, 1.0, 100, 100, ccolor::RED)));
    window.add_object(my_box);

    let platform = Rc::new(RefCell::new(get_platform(0.0, 870.0, 1700, 50)));
    window.add_object(platform);

    let platform = Rc::new(RefCell::new(get_platform(400.0, 300.0, 300, 100)));
    window.add_object(platform);
    // let my_box = Rc::new(RefCell::new(get_platform(0, 500, 1000, 50)));
    // window.add_object(my_box);d

    while window.is_open() {
        window.poll_input();
        window.update();

        window.fill(ccolor::BLACK);
        window.draw(&image);
        window.show_window();
    }
}
