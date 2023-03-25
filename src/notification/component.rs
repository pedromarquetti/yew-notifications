use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

use crate::utils::format_date_time;
use crate::{Notifiable, Notification};

#[derive(Properties, Clone, PartialEq)]
pub struct NotificationComponentProps {
    pub notification: Notification,
    pub onclick: Callback<MouseEvent>,
    pub onenter: Callback<MouseEvent>,
    pub onleave: Callback<MouseEvent>,
}

#[function_component(NotificationComponent)]
pub fn notification_component(props: &NotificationComponentProps) -> Html {
    let title = props.notification.title.as_ref();
    let text = &props.notification.text;
    let notification_type = &props.notification.notification_type;
    let spawn_time = &props.notification.spawn_time;

    let onclick = props.onclick.clone();
    let onenter = props.onenter.clone();
    let onleave = props.onleave.clone();

    let mut classes = vec![classes!("notification"), notification_type.into()];
    if props.notification.is_paused() {
        classes.push(classes!("paused"));
    }

    html! {
        <div {onclick} onmouseenter={onenter} onmouseleave={onleave} class={classes}>
            {if let Some(title) = title {
                html! { <span class={classes!("notification-title")}>{title}</span> }
            } else {
                html! {}
            }}
            <span>{text}</span>
            <span class={classes!("time")}>{format_date_time(spawn_time)}</span>
        </div>
    }
}
