use buaa_api::time::DateTime;
use dioxus::prelude::*;

// TODO: 今日课程, 今日博雅, 自定义日程. 博雅进度, 博雅推荐. 考试信息安排, 阳光体育情况, 独立刷新, 卡片化

#[component]
pub fn Dashboard() -> Element {
    ClassCard()
}

#[component]
fn ClassCard() -> Element {
    use buaa_api::api::class::Schedule;

    let mut schedule = use_signal(|| Option::<Schedule>::None);
    let mut current = use_signal(|| 0usize);

    // 获取当日课程
    let fetch = use_resource(move || {
        async move {
            // 每次刷新课程列表都应该重置 current 索引
            current.set(0);
            let class = crate::ctx().class();
            // let now = DateTime::now();
            let now = DateTime::parse("2026-03-20 00:00").unwrap();
            // 拿不到暂时返回空列表, 实际上这是错误码为 2 的情况, 应交给上层 API 处理
            class.query_schedule(&now).await.unwrap_or_default()
        }
    });

    // 根据 current 索引更新 schedule 信号
    use_effect(move || {
        // 这里会订阅 current 索引, 每当 current 变化时都会重新执行这个 effect, 从而更新 schedule 信号
        let index = current();
        match &*fetch.read() {
            Some(schedules) => {
                // 如果索引超出课程列表长度, 则说明没有更多课程了, 将 schedule 设为 None
                if index >= schedules.len() {
                    schedule.set(None);
                    return;
                }
                // 如果课程状态已完成就检查下一门
                if schedules[index].status {
                    current.set(index + 1);
                    println!("Current schedule: {}", schedules[index].name);
                    return;
                }
                // 能走到这里的就是未超出索引并且课程未完成的情况了, 将 schedule 设为当前课程
                schedule.set(Some(schedules[index].clone()));
            }
            _ => (),
        }
    });

    // TODO: 还需处理时间
    let checkin_handle = move |_| {
        match &*schedule.read() {
            Some(s) => {
                let id = s.id.clone();
                spawn(async move {
                    let class = crate::ctx().class();
                    class.checkin(&id).await.unwrap();
                    // 如果签到成功就切到下一个课程
                    current += 1;
                });
            }
            _ => {
                return;
            }
        }
    };

    match schedule() {
        Some(s) => rsx!(
            div {
                class: "class-card",
                div {
                    class: "class-name",
                    "{s.name}"
                }
                div {
                    class: "class-time",
                    "{s.time}"
                }
                button {
                    class: "checkin-btn",
                    onclick: checkin_handle,
                    "签到"
                }
            }
        ),
        None => rsx!(),
    }
}
