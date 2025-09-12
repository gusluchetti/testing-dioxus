use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
    }
}

#[derive(serde::Deserialize, Debug)]
struct BtsPhl {
    #[serde(rename = "type")]
    typ: String,
    last_updated: String,
    features: Vec<Feature>,
}

#[derive(serde::Deserialize, Debug)]
struct Feature {
    #[serde(rename = "type")]
    typ: String,
    properties: Vec<Property>,
}

#[derive(serde::Deserialize, Debug)]
struct Property {
    id: u64,
    name: String,
}

#[component]
pub fn Hero() -> Element {
    let fetch_new = move |_: Event<MouseData>| async move {
        let response = reqwest::get("https://bts-status.bicycletransit.workers.dev/phl")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{:?}", response);
    };

    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            button { id: "fetch", onclick: fetch_new }
        }
    }
}
