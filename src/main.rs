mod common;
mod qml_types;
mod day1;
mod day2;

use qmetaobject::{prelude::*, QUrl};
use qml_types::register_all_qml_types;

fn init_qrc() {
    qrc!(compile_qml_files, "qml" {
        "resources/qml/mainwindow.qml" as "mainwindow"
    });
    compile_qml_files();
}

fn main() {
    register_all_qml_types();
    init_qrc();
    let mainwindow_url = QUrl::from(QString::from("qrc:/qml/mainwindow"));
    let mut engine = QmlEngine::new();
    engine.load_url(mainwindow_url);
    engine.exec();
}
