use adw::{gtk::License, prelude::*, AboutWindow, Application};

pub fn about_window(app: &Application) {
    let window = AboutWindow::builder()
        .application(app)
        .application_name("Celeste")
        .copyright("© 2022 Hunter Wittenborn")
        .developer_name("Hunter Wittenborn")
        .icon_name("com.hunterwittenborn.Celeste")
        .issue_url("https://github.com/hwittenborn/celeste")
        .license_type(License::Gpl30)
        .support_url("https://github.com/hwittenborn/celeste/issues")
        .build();

    window.show();
}
