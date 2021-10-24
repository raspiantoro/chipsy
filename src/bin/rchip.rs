use rchip::{machine::Machine, ui::UI};

static UI_WIDHT: u32 = 800;
static UI_HEIGHT: u32 = 600;

fn main() {
    let ui = UI::new(UI_WIDHT, UI_HEIGHT);
    ui.show();

    let mut machine = Machine::new();
    machine.init();
}
