use cursive::{
    event::Event::CtrlChar,
    Cursive,
};
use woroxide::*;

fn main() {
    // Cursive
    let mut siv = Cursive::default();
    let default_phase1 = Phase1 {
        purpose: RECEIVE_PURPOSE::Memorize,
        kind: RECEIVE_KIND::Word,
        range: RECEIVE_RANGE::Chap,
        method: RECEIVE_METHOD::Rand,
    };
    siv.set_user_data(default_phase1);
    siv.add_global_callback(CtrlChar('q'), |s| s.quit());

    // Start
    setting(&mut siv);

    siv.run();
}
