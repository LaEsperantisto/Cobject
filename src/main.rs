use cobject::ckey::CKey;
use cobject::cmouse::CMouse;
use cobject::cobject::CObject;
use cobject::cpoint::CPoint;
use cobject::cwindow::CWindow;

fn main() {
    let mut window = CWindow::new(640, 480, String::from("Test"));

    let obj = CObject::new(100, 100, 100, 100, 0x0000FF00);
    while window.is_open() {
        if window.is_pressed(CKey::Escape) {
            break;
        }
        let mouse_pos = window.mouse_pos();
        if window.point_collides(&CPoint::from(mouse_pos), &obj) && window.is_clicked(CMouse::Left)
        {
            break;
        };

        window.draw(&obj);
        window.update();
    }
}
