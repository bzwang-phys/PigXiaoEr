// use crate::setting;
#[cfg(not(target_os = "android"))]
use crate::systemtray;
use slint::{ComponentHandle, SharedString, Timer};
slint::include_modules!();
use std::cell::RefCell;
use std::rc::Rc;

pub struct MainWindow {
    pic_list: Vec<slint::Image>,
    pic_index: usize,
    timer: Timer,
    // systemtray: systemtray::SystemTray,
}

impl MainWindow {
    pub fn new() -> Result<MainWindowUi, slint::PlatformError> {
        let ui = MainWindowUi::new()?;
        let timer = Timer::default();

        // 将所有状态和逻辑封装在 Rc<RefCell<...>> 中
        let state = Rc::new(RefCell::new(Self {
            pic_list: vec![], // 在这里加载或初始化图片列表
            pic_index: 0,
            timer, // 将 timer 的所有权移入 state
            // systemtray: systemtray::SystemTray::new(),
        }));

        // 连接 UI 回调到我们的状态逻辑
        // 使用 .clone() 来为每个回调创建一个新的 Rc 引用

        // --- 示例：连接一个名为 `next_picture` 的回调 ---
        // ui.on_next_picture({
        //     let state = state.clone();
        //     move || {
        //         state.borrow_mut().show_next_picture();
        //     }
        // });

        // --- 连接退出应用的回调 ---
        ui.on_quit_app({
            let state = state.clone();
            // 获取一个弱引用到 UI 窗口，用于关闭
            let ui_handle = ui.as_weak();
            move || {
                // 调用 state 中的方法来处理退出前的清理工作
                state.borrow().prepare_to_quit();
                // 使用弱引用来隐藏窗口或退出事件循环
                ui_handle.unwrap().hide().unwrap();
                // 或者 slint::quit_event_loop();
            }
        });

        // --- 连接其他回调 ---
        ui.on_show_window({
            let ui_handle = ui.as_weak();
            move || {
                let ui = ui_handle.unwrap();
                ui.show().unwrap();
                println!("显示窗口");
            }
        });

        ui.on_hide_window({
            let ui_handle = ui.as_weak();
            move || {
                let ui = ui_handle.unwrap();
                ui.hide().unwrap();
                println!("隐藏窗口");
            }
        });

        ui.on_settings_clicked({
            let ui_handle = ui.as_weak();
            move || {
                let ui = ui_handle.unwrap();
                ui.set_home_text("设置已点击".into());
                println!("设置按钮被点击");
            }
        });

        ui.on_pause_clicked({
            let state = state.clone();
            let ui_handle = ui.as_weak();
            move || {
                let ui = ui_handle.unwrap();
                let mut state_mut = state.borrow_mut();
                state_mut.toggle_pause(&ui);
            }
        });

        ui.on_exit({
            let state = state.clone();
            let ui_handle = ui.as_weak();
            move || {
                let ui = ui_handle.unwrap();
                state.borrow().prepare_to_quit();
                ui.hide().unwrap();
                println!("退出应用");
            }
        });


        // 启动计时器
        state.borrow().timer.start(
            slint::TimerMode::Repeated,
            std::time::Duration::from_secs(5),
            {
                let state = state.clone();
                move || {
                    state.borrow_mut().show_next_picture();
                }
            },
        );

        // 返回 UI 句柄，以便 main 函数可以调用 .run()
        Ok(ui)
    }

    // 业务逻辑函数，被回调调用
    pub fn show_next_picture(&mut self) {
        if self.pic_list.is_empty() {
            return;
        }
        self.pic_index = (self.pic_index + 1) % self.pic_list.len();
        println!("显示下一张图片，索引: {}", self.pic_index);
        // 假设 UI 上有一个 `current_image` 属性
        // self.ui.set_current_image(self.pic_list[self.pic_index].clone());
    }

    pub fn prepare_to_quit(&self) {
        println!("正在准备退出...");
        // 在这里可以执行保存状态等操作
        // self.systemtray.cleanup(); // 假设有清理方法
    }

    // 切换暂停状态
    pub fn toggle_pause(&mut self, ui: &MainWindowUi) {
        // 这里可以实现暂停/恢复逻辑
        // 例如停止/启动计时器
        ui.set_home_text("暂停/恢复".into());
        println!("切换暂停状态");
    }
}
