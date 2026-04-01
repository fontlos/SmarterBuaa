use buaa_api::time::DateTime;
use dioxus::prelude::*;

// TODO: 今日课程, 今日博雅, 自定义日程. 博雅进度, 博雅推荐. 考试信息安排

#[component]
pub fn Dashboard() -> Element {
    ScheduleList()
}

#[component]
pub fn ScheduleList() -> Element {
    let class_schedule = use_resource(move || {
        async move {
            let class = crate::ctx().class();
            let now = DateTime::now();
            let res = class.query_schedule(&now).await?;
            buaa_api::Result::Ok(res)
        }
    });

    let schedule_list = match &*class_schedule.read() {
        Some(Ok(r)) => {
            let list = r.iter().map(|c| {
                rsx!(
                    div {
                        class: "schedule-item",
                        div {
                            class: "schedule-name",
                            "{c.name}"
                        }
                        div {
                            class: "schedule-info",
                            div {
                                class: "schedule-time",
                                "{c.time}"
                            }
                            div {
                                class: "schedule-type",
                                "Class"
                            }
                        }
                    }
                )
            });
            rsx!({ list })
        }
        Some(Err(e)) => rsx!(
            div {
                "Error: {e}"
            }
        ),
        None => ScheduleSkeleton(),
    };

    rsx! {
        div {
            class: "schedule-container",
            { schedule_list }
        }
    }
}

#[component]
fn SkeletonLine(width: &'static str) -> Element {
    rsx!(
        div {
            class: "skeleton",
            style: "width: {width}; height: 16px; margin-bottom: 4px;"
        }
    )
}

#[component]
fn ScheduleSkeleton() -> Element {
    // 通常显示 5-6 个占位项，模拟列表长度
    let count = 6;

    rsx! {
        for _ in 0..count {
            div { class: "course-item",
                SkeletonLine { width: "60%" }
                SkeletonLine { width: "40%" }
            }
        }
    }
}
