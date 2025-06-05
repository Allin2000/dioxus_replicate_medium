// use dioxus::prelude::*;
// use serde::Deserialize;
// use crate::api::tag::fetch_tags;


// #[component]
// pub fn TagList() -> Element {

//         let tags = use_resource(|| async {
//             fetch_tags().await.map(|tags| tags)
//         });

//     // 根据加载状态渲染不同内容
//     // !! 这里是修复点 !!
//     // tags.value() 返回 ReadOnlySignal，需要调用它来获取实际值
//     match tags.value()() { // <-- 在 tags.value() 后面加上 () 来调用信号量
//         // 数据加载完成，并成功获取 Some(Some(tags))
//         Some(Some(tag_list)) => {
//             // 这里需要返回一个 Element。如果 tag_list 是空的，返回一个段落；否则返回由标签链接组成的列表。
//             if tag_list.is_empty() {
//                  rsx! { p { "No popular tags yet." } }
//             } else {
//                  rsx! {
//                     // 渲染标签列表，使用 {} 包裹迭代器
//                     { // <-- 包裹迭代器表达式的花括号
//                         tag_list.iter().map(|tag| rsx! {
//                             a {
//                                 class: "tag-pill tag-default",
//                                 href: "#", // 临时使用 # 或 ""，后面会处理点击事件
//                                 key: "{tag}", // 为列表项添加 key
//                                 "{tag}"
//                             }
//                         })
//                     } // <-- 闭合花括号
//                  }
//             }
//         },
//         // 数据加载完成，但 use_resource 闭包内部因为某个 .ok()? 失败而返回了 None
//         // resource 的状态值是 Some(None)
//         Some(None) => rsx!(p { "Failed to load tags." }),
//         // Resource 处于 Pending 状态 (初始状态或重新加载中)
//         // resource 的状态值是 None
//         None => rsx!(p { "Loading tags..." }),
//     }
// }



// src/components/taglist.rs
use dioxus::prelude::*;
use serde::Deserialize;
use crate::api::tag::fetch_tags;

#[derive(Props, Clone, PartialEq)]
pub struct TagListProps {
    pub on_tag_click: EventHandler<String>,
}

#[component]
pub fn TagList(props: TagListProps) -> Element {
    let tags = use_resource(|| async {
        fetch_tags().await.map(|tags| tags)
    });

    // 根据加载状态渲染不同内容
    match tags.value()() {
        // 数据加载完成，并成功获取 Some(Some(tags))
        Some(Some(tag_list)) => {
            // 这里需要返回一个 Element。如果 tag_list 是空的，返回一个段落；否则返回由标签链接组成的列表。
            if tag_list.is_empty() {
                rsx! { p { "No popular tags yet." } }
            } else {
                rsx! {
                    // 渲染标签列表，使用 {} 包裹迭代器
                    {
                        tag_list.iter().map(|tag| {
                            let tag_clone = tag.clone();
                            rsx! {
                                a {
                                    class: "tag-pill tag-default",
                                    href: "#",
                                    key: "{tag}",
                                    onclick: move |_| {
                                        props.on_tag_click.call(tag_clone.clone());
                                    },
                                    "{tag}"
                                }
                            }
                        })
                    }
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


