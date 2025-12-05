// Android 平台入口点（cdylib）

mod log_capture;
mod mainwindow;

slint::include_modules!();

use GaiaNet::gaia_nets::daemon_server::DaemonServer;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    // 1. 创建共享的日志缓冲区
    let log_buffer = mainwindow::new_log_buffer();

    // 2. 初始化日志捕获系统
    let _ = mainwindow::init_log_capture(log_buffer.clone(), 200);

    // 3. 启动 DaemonServer
    let mut daemon_server = DaemonServer::new();
    daemon_server.run("shell", "net.conf");

    slint::android::init(app).unwrap();

    // 4. 创建带日志缓冲区的主窗口
    let main_win = mainwindow::MainWindow::new_with_log_buffer(log_buffer).unwrap();
    main_win.run().unwrap();
}
