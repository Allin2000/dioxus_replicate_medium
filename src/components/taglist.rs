use dioxus::prelude::*;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct TagResponse {
    tags: Vec<String>,
}

#[component]
pub fn TagList() -> Element {
    // 异步加载 tags 数据
    // use_resource 返回一个 Resource，其 state 是 ReadOnlySignal<Option<Result<T, E>>> (或类似，取决于闭包返回类型)
    // 在你的代码中，闭包返回的是 Option<Vec<String>>，所以 Resource 最终的状态值是 ReadOnlySignal<Option<Option<Vec<String>>>>
    let tags = use_resource(|| async {
        let client = reqwest::Client::new();
        let response = client
            .get("http://localhost:8000/api/tags")
            .send()
            .await
            .ok()? // 处理 send/await 可能的错误，将 Err 转换为 None，从而导致 use_resource 的值为 Some(None)
            .json::<TagResponse>()
            .await // json() 也是异步的，需要 await
            .ok()?; // 处理 json/await 可能的错误，将 Err 转换为 None，从而导致 use_resource 的值为 Some(None)
        Some(response.tags) // 如果所有步骤都成功，返回 Some(Vec<String>)，导致 use_resource 的值为 Some(Some(Vec<String>))
    });

    // 根据加载状态渲染不同内容
    // !! 这里是修复点 !!
    // tags.value() 返回 ReadOnlySignal，需要调用它来获取实际值
    match tags.value()() { // <-- 在 tags.value() 后面加上 () 来调用信号量
        // 数据加载完成，并成功获取 Some(Some(tags))
        Some(Some(tag_list)) => {
            // 这里需要返回一个 Element。如果 tag_list 是空的，返回一个段落；否则返回由标签链接组成的列表。
            if tag_list.is_empty() {
                 rsx! { p { "No popular tags yet." } }
            } else {
                 rsx! {
                    // 渲染标签列表，使用 {} 包裹迭代器
                    { // <-- 包裹迭代器表达式的花括号
                        tag_list.iter().map(|tag| rsx! {
                            a {
                                class: "tag-pill tag-default",
                                href: "#", // 临时使用 # 或 ""，后面会处理点击事件
                                key: "{tag}", // 为列表项添加 key
                                "{tag}"
                            }
                        })
                    } // <-- 闭合花括号
                 }
            }
        },
        // 数据加载完成，但 use_resource 闭包内部因为某个 .ok()? 失败而返回了 None
        // resource 的状态值是 Some(None)
        Some(None) => rsx!(p { "Failed to load tags." }),
        // Resource 处于 Pending 状态 (初始状态或重新加载中)
        // resource 的状态值是 None
        None => rsx!(p { "Loading tags..." }),
    }
}
