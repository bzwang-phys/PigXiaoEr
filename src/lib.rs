// mod infobox;
mod mainwindow;
// mod setting;
// mod slogan;

#[cfg(not(target_os = "android"))]
mod systemtray;

slint::include_modules!();

#[cfg(target_os = "android")]
use slint::android::{AndroidApp, *};

#[cfg(not(target_os = "android"))]
#[no_mangle]
pub extern "C" fn wokankan() {}

#[cfg(not(target_os = "android"))]
#[no_mangle]
pub extern "C" fn niseesee(a: i32) -> f32 {
    a as f32
}

#[cfg(not(target_os = "android"))]
#[no_mangle]
pub extern "C" fn woshimain(a: std::os::raw::c_schar) -> Result<(), slint::PlatformError> {
    let main_win = mainwindow::MainWindow::new()?;
    main_win.window().set_fullscreen(true);
    main_win.run()?;
    // let tray = systemtray::SystemTray::new();
    //
    // tray.show();
    Ok(())
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let main_win = mainwindow::MainWindow::new().unwrap();
    main_win.run().unwrap();
}
