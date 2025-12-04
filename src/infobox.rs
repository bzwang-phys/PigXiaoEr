use slint::{ComponentHandle, SharedString};

slint::slint! {
    export component InfoBoxUi inherits Window {
        in property <string> text;
        callback hide_window();
        callback open_setting();
        callback exit();
        Rectangle {
            background: black;
            border-radius: 16px;
            Text {
                text: root.text;
                color: red;
                font-size: 50px;
                horizontal-alignment: left;
                vertical-alignment: top;
                wrap: true;
                padding: 13px;
            }
        }
        // 菜单按钮
        PopupMenu {
            menu {
                MenuItem { text: "Hide"; clicked => { root.hide_window(); } }
                MenuItem { text: "Setting"; clicked => { root.open_setting(); } }
                MenuItem { text: "Exit"; clicked => { root.exit(); } }
            }
        }
    }
}

pub struct InfoBox {
    ui: slint::Weak<InfoBoxUi>,
}

impl InfoBox {
    pub fn new(text: &str) -> Self {
        let ui = InfoBoxUi::new().into_weak();
        if let Some(ui) = ui.upgrade() {
            ui.set_text(SharedString::from(text));
        }
        InfoBox { ui }
    }

    pub fn set_text(&self, text: &str) {
        if let Some(ui) = self.ui.upgrade() {
            ui.set_text(SharedString::from(text));
        }
    }

    pub fn show(&self) {
        if let Some(ui) = self.ui.upgrade() {
            ui.show();
        }
    }
}