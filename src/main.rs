use qmetaobject::*;


qrc!(my_resource,
    "sql_tryer" {
        "qml/main.qml"
    },
);

fn main() {
    my_resource();
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/sql_tryer/qml/main.qml".into());
    engine.exec();

}