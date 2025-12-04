```rust
use slint::{ComponentHandle};

slint::slint! {
    export component SloganUi inherits Dialog {
        Text { text: "Slogan Settings"; }
        // 可添加更多控件
        Button { text: "Close"; clicked => { root.hide(); } }
    }
}

pub struct Slogan {
    ui: slint::Weak<SloganUi>,
}

impl Slogan {
    pub fn new() -> Self {
        let ui = SloganUi::new().into_weak();
        Slogan { ui }
    }

    pub fn show(&self) {
        if let Some(ui) = self.ui.upgrade() {
            ui.show();
        }
    }
}
```