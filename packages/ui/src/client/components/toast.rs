use dioxus::prelude::*;

pub const TOAST_TIMEOUT_MS: u32 = 2_000;

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub id: u64,
    pub message: String,
    pub tone: ToastTone,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastTone {
    Info,
    Success,
    Error,
}

#[component]
pub fn ToastRegion(mut toast: Signal<Option<Toast>>) -> Element {
    use_effect(move || {
        if let Some(active_toast) = toast() {
            spawn(async move {
                wait_for_toast_timeout().await;

                if toast.peek().as_ref().map(|toast| toast.id) == Some(active_toast.id) {
                    toast.set(None);
                }
            });
        }
    });

    rsx! {
        div {
            class: "toast-region pointer-events-none absolute left-1/2 top-[calc(50%+100px)] z-20 flex w-[min(420px,calc(100vw_-_32px))] -translate-x-1/2 -translate-y-1/2 items-center justify-center max-md:w-[min(376px,calc(100vw_-_32px))]",
            aria_live: "polite",
            if let Some(toast) = toast() {
                div { class: toast_class(toast.tone), "{toast.message}" }
            }
        }
    }
}

fn toast_class(tone: ToastTone) -> &'static str {
    match tone {
        ToastTone::Info => "toast max-w-full overflow-hidden text-ellipsis whitespace-nowrap rounded-lg border border-[var(--app-accent)] bg-[var(--app-surface-strong)] px-4 py-2 text-sm leading-[1.35] text-[var(--app-accent-strong)] shadow-[var(--app-shadow-soft)]",
        ToastTone::Success => "toast max-w-full overflow-hidden text-ellipsis whitespace-nowrap rounded-lg border border-[var(--app-positive)] bg-[var(--app-surface-strong)] px-4 py-2 text-sm leading-[1.35] text-[var(--app-positive)] shadow-[var(--app-shadow-soft)]",
        ToastTone::Error => "toast max-w-full overflow-hidden text-ellipsis whitespace-nowrap rounded-lg border border-[var(--app-negative)] bg-[var(--app-surface-strong)] px-4 py-2 text-sm leading-[1.35] text-[var(--app-negative)] shadow-[var(--app-shadow-soft)]",
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn wait_for_toast_timeout() {
    gloo_timers::future::TimeoutFuture::new(TOAST_TIMEOUT_MS).await;
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn wait_for_toast_timeout() {
    futures_timer::Delay::new(std::time::Duration::from_millis(TOAST_TIMEOUT_MS.into())).await;
}
