use buaa_api::Context;
use buaa_api::time::DateTime;
use dioxus::prelude::*;

#[component]
pub fn SchedulePage() -> Element {
    let ctx = use_context::<std::rc::Rc<Context>>();

    let res = use_resource(move || {
        let ctx = ctx.clone();
        async move {
            let class = ctx.class();
            let now = DateTime::now();
            let res = class.query_schedule(&now).await?;
            buaa_api::Result::Ok(res)
        }
    });

    match &*res.read() {
        Some(Ok(r)) => {
            let list = r.iter().map(|c| {
                rsx!(
                    div {
                        div {
                            class: "course-name",
                            "{c.name}"
                        }
                        div {
                            class: "course-time",
                            "{c.time}"
                        }
                    }
                )
            });
            rsx!(
                div{
                    class: "schedule-container",
                    { list }
                }
            )
        }
        Some(Err(e)) => rsx!(
            div {
                "Error: {e}"
            }
        ),
        None => rsx!(
            div {
                "Loading..."
            }
        ),
    }
}
