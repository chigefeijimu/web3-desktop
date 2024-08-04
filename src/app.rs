use leptos::*;
use leptos::ev::Event;
use leptos::leptos_dom::ev::SubmitEvent;
use web_sys::HtmlInputElement;
use web_sys::FileList;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            let args = to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let (files_read, files_write) = create_signal(Vec::new());

    let on_change = move |event: Event| {
        let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();

        if let Some(file_list) = input.files() {
            // 获取文件列表
            let selected_files = (0..file_list.length())
                .filter_map(|i| file_list.item(i))
                .map(|file| file.name())
                .collect::<Vec<_>>();

            files_write.set(selected_files)
        }
    };

    view! {
        <main class="container">
            <div class="row">
                <a href="https://www.baidu.com" target="_blank">
                    <img src="public/logo.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
            </div>

            <p>"欢迎来到web3的世界！"</p>

            <form class="row" id="mainForm" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="请输入想要的地址前缀"
                    on:input=update_name
                />
                <input
                    id="file-input"
                    type="file"
                    placeholder="请选择钱包文件夹"
                    on:change=on_change
                />
                <button type="submit">"创建钱包"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p>
        </main>
    }
}
