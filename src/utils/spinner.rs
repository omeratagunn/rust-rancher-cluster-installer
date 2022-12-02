use terminal_spinners::{SpinnerBuilder, DOTS, SpinnerHandle};

pub(crate) fn spinner(text: String) -> SpinnerHandle {
    let handle = SpinnerBuilder::new().spinner(&DOTS).text(text).start();
    return handle
}
