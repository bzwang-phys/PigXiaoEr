use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, TrayIconEvent,
};

pub struct SystemTray {
    // This struct can be empty as the tray icon's lifecycle is managed elsewhere
}

impl SystemTray {
    pub fn new() -> Self {
        let menu = Box::new(Menu::new());

        let show_window_i = MenuItem::new("显示窗口", true, None);
        let quit_i = MenuItem::new("退出", true, None);

        menu.append_items(&[&show_window_i, &PredefinedMenuItem::separator(), &quit_i])
            .unwrap();

        let tray_icon = TrayIconBuilder::new()
            .with_menu(menu)
            .with_tooltip("PigXiaoEr")
            .build()
            .unwrap();

        let menu_channel = TrayIconEvent::receiver();

        // std::thread::spawn(move || {
        //     while let Ok(event) = menu_channel.recv() {
        //         let ui_clone = ui.clone();
        //         if event.id == show_window_i.id() {
        //             slint::invoke_from_event_loop(move || {
        //                 ui_clone.unwrap().show().unwrap();
        //             })
        //             .unwrap();
        //         } else if event.id == quit_i.id() {
        //             slint::invoke_from_event_loop(move || {
        //                 slint::quit_event_loop().unwrap();
        //             })
        //             .unwrap();
        //             break;
        //         }
        //     }
        // });

        // Keep the tray icon alive for the duration of the program
        std::mem::forget(tray_icon);

        SystemTray {}
    }
}
