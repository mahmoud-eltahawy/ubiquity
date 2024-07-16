use leptos::{html::div, prelude::*};

pub fn container(children: impl IntoView) -> impl IntoView {
    div()
        .attr("class", "my-auto border border-base-content rounded-xl p-4 pb-8 2xl:w-[65%] xl:w-[72.5%] lg:w-[80%] md:w-[87.5%] sm-[95%] w-[98%] h-[calc(100dvh-7.25rem)]")
        .child(children)
}

pub fn style_container(children: impl IntoView) -> impl IntoView {
    div()
        .class("h-[calc(100vh-4rem)] flex flex-col content-center align-center items-center justify-center")
        .child(children)
}
