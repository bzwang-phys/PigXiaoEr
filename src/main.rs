// mod infobox;
mod mainwindow;
// mod setting;
// mod slogan;
#[cfg(not(target_os = "android"))]
mod systemtray;

slint::include_modules!();

use slint::SharedString;
use std::rc::Rc;
use GaiaNet::gaia_nets::daemon_server::DaemonServer;

#[cfg(not(target_os = "android"))]
fn main() -> Result<(), slint::PlatformError> {
    // let mut daemon_server = DaemonServer::new();
    // daemon_server.run("shell", "net.conf");

    let main_win = mainwindow::MainWindow::new()?;
    main_win.window().set_fullscreen(true);
    main_win.run()?;
    // let tray = systemtray::SystemTray::new();
    // tray.show();

    Ok(())
}

#[cfg(target_os = "android")]
use slint::android::{AndroidApp, *};
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    let mut daemon_server = DaemonServer::new();
    daemon_server.run("shell", "net.conf");
    slint::android::init(app).unwrap();

    let main_win = mainwindow::MainWindow::new().unwrap();
    main_win.run().unwrap();
}
