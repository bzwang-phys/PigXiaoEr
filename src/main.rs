mod log_capture;
mod mainwindow;
// mod setting;
mod systemtray;

slint::include_modules!();

use GaiaNet::gaia_nets::daemon_server::DaemonServer;

fn main() -> Result<(), slint::PlatformError> {
    // 1. 创建共享的日志缓冲区
    let log_buffer = mainwindow::new_log_buffer();

    // 2. 初始化日志捕获系统（捕获 log::info! 等宏）
    if let Err(e) = mainwindow::init_log_capture(log_buffer.clone(), 200) {
        eprintln!("警告：无法初始化日志捕获: {}", e);
    }

    // 3. 启动 DaemonServer（它的 log::info! 输出会被捕获）
    let mut daemon_server = DaemonServer::new();
    daemon_server.run("shell", "net.conf");

    // 4. 创建带日志缓冲区的主窗口
    let main_win = mainwindow::MainWindow::new_with_log_buffer(log_buffer)?;
    main_win.window().set_fullscreen(true);
    main_win.run()?;
    // let tray = systemtray::SystemTray::new();
    // tray.show();

    Ok(())
}
