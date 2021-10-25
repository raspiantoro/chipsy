use rchip::{machine::Machine, ui::UI};

static WINDOW_WIDHT: u32 = 1000;
static WINDOW_HEIGHT: u32 = 500;
static UI_SCALE: u32 = 10;

fn main() {
    let ui = UI::new(WINDOW_WIDHT, WINDOW_HEIGHT);
    ui.show();

    let mut machine = Machine::new();
    machine.init();

    machine.mem_assign(0x200, 0x2D);
    machine.mem_assign(0x201, 0x3A);
    machine.mem_assign(0x202, 0x00);
    machine.mem_assign(0x203, 0x00);

    machine.run();
}
