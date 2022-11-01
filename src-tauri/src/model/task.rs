#[skip_serializing_none]
#[derive(Serialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Task {
    pub id: String,
    pub ctime: String,
    pub doc_id: String,

    pub done: bool,
    pub title: String,
    pub desc: Option<String>,
}