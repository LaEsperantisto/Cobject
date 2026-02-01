use cobject::{ccolor, get_platform, CBoxCannon, CPlayer, CWindow};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut window = CWindow::new(1700, 900, "Test".into());
    window.init();

    let player = Rc::new(RefCell::new(CPlayer::new(100.0, 400.0)));

    window.add_input_listener(player.clone());
    window.add_object(player.clone());

    let my_box = Rc::new(RefCell::new(CBoxCannon::new(
        900.0,
        1.0,
        100.0,
        100.0,
        ccolor::BLUE,
    )));
    window.add_object(my_box.clone());
    window.add_input_listener(my_box.clone());
    let my_box = Rc::new(RefCell::new(CBoxCannon::new(
        200.0,
        1.0,
        100.0,
        100.0,
        ccolor::BLUE,
    )));
    window.add_object(my_box.clone());
    window.add_input_listener(my_box.clone());

    let my_box = Rc::new(RefCell::new(CBoxCannon::new(
        500.0,
        1.0,
        100.0,
        100.0,
        ccolor::RED,
    )));
    window.add_object(my_box.clone());
    window.add_input_listener(my_box.clone());
    let my_box = Rc::new(RefCell::new(CBoxCannon::new(
        500.0,
        1.0,
        100.0,
        100.0,
        ccolor::RED,
    )));
    window.add_object(my_box.clone());
    window.add_input_listener(my_box.clone());

    let platform = Rc::new(RefCell::new(get_platform(0.0, 850.0, 1650.0, 50.0)));
    window.add_object(platform);

    let platform = Rc::new(RefCell::new(get_platform(400.0, 300.0, 300.0, 100.0)));
    window.add_object(platform);

    while window.is_open() {
        window.poll_input();
        window.update();

        window.fill(ccolor::BLACK);
        window.show_window();
    }
}
