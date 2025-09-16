use dioxus::{logger::tracing, prelude::*};
use serde_json::{Map, Value};

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
    last_updated: String,
    features: Vec<Feature>,
    #[serde(rename = "type")]
    typ: String,
}

#[derive(serde::Deserialize, Debug)]
struct Feature {
    geometry: Geometry,
    properties: Map<String, Value>,
    #[serde(rename = "type")]
    typ: String,
}

#[derive(serde::Deserialize, Debug)]
struct Geometry {
    coordinates: [f64; 2],
    #[serde(rename = "type")]
    typ: String,
}

#[component]
pub fn Hero() -> Element {
    let mut json_features: Signal<Vec<Feature>> = use_signal(|| vec![]);

    let fetch_new = move |_: Event<MouseData>| async move {
        let response = reqwest::get("https://bts-status.bicycletransit.workers.dev/phl")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let json: BtsPhl = serde_json::from_str(&response).unwrap();
        json_features.set(json.features);
    };

    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            button { id: "fetch", onclick: fetch_new }
            ul {
                for feat in json_features.iter() {
                    li { "{feat:?}" }
                }
            }
        }
    }
}
