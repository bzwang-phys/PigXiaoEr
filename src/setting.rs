use slint::{ComponentHandle, SharedString};

slint::slint! {
    export component SettingUi inherits Dialog {
        in property <int> timer1;
        in property <int> timer2;
        callback save_setting(int, int);

        TextInput { id: timer1_input; text: timer1; }
        TextInput { id: timer2_input; text: timer2; }
        Button { text: "OK"; clicked => { root.save_setting(timer1_input.text, timer2_input.text); } }
        Button { text: "Cancel"; clicked => { root.hide(); } }
    }
}

pub struct Setting {
    ui: slint::Weak<SettingUi>,
    timer1: i32,
    timer2: i32,
}

impl Setting {
    pub fn new(timer1: i32, timer2: i32) -> Self {
        let ui = SettingUi::new().into_weak();
        let mut setting = Setting { ui, timer1, timer2 };
        if let Some(ui) = setting.ui.upgrade() {
            ui.set_timer1(timer1);
            ui.set_timer2(timer2);
            ui.on_save_setting(move |t1, t2| {
                // 保存设置逻辑
            });
        }
        setting
    }

    pub fn show(&self) {
        if let Some(ui) = self.ui.upgrade() {
            ui.show();
        }
    }
}