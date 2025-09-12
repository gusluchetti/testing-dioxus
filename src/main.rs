use dioxus::prelude::*;
use serde::Deserialize;

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

#[derive(serde::Deserialize)]
struct BtsPhl {
    #[serde(rename = "type")]
    typ: String,
    last_updated: String,
    features: Vec<Feature>,
}

#[derive(serde::Deserialize)]
struct Feature {
    #[serde(rename = "type")]
    typ: String,
    properties: Vec<Property>,
}

#[derive(serde::Deserialize)]
struct Property {
    id: u64,
    name: String,
}

#[component]
pub fn Hero() -> Element {
    use_future(|_| async move {
        let response = reqwest::get("https://bts-status.bicycletransit.workers.dev/phl")
            .await?
            .text()
            .await?;
        let json: BtsPhl = serde_json::from_str(&response)?;
        dbg!(json);
    });

    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            // div {
            //     p { "something else entirely" }
            // }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
