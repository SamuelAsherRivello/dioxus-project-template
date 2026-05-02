use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::client::components::toast::{Toast, ToastTone};
use crate::client::models::{TemplateDataLoadRequest, TemplateDataLoadResult, TemplateDataSource};
use crate::client::services::template_data_service::{load_template_data, refresh_template_data};

const PAGE_CLASS: &str = "mx-auto min-w-0 w-[min(1120px,calc(100%_-_40px))] py-[42px] pb-14 max-md:w-[min(calc(100%_-_56px),1120px)] max-md:py-[30px] max-md:pb-[42px]";
const TITLE_CLASS: &str = "mb-[18px] mt-0 text-[clamp(36px,6vw,64px)] leading-none tracking-normal text-[var(--app-text)] max-md:text-[clamp(34px,12vw,54px)]";
const DATA_CLASS: &str = "mb-6 inline-flex min-h-10 items-center rounded-lg border border-[var(--app-border)] bg-[var(--app-surface)] px-3.5 py-2 text-base font-bold text-[var(--app-accent-strong)] shadow-[var(--app-shadow-soft)]";
const BODY_CLASS: &str = "grid min-w-0 w-full gap-[18px]";
const BODY_TEXT_CLASS: &str = "m-0 min-w-0 max-w-full break-words text-[19px] leading-[1.65] text-[var(--app-text-muted)] max-md:max-w-[30ch] max-md:text-[17px] max-md:leading-[1.58]";
const WARNING_CLASS: &str = "font-extrabold text-[var(--app-negative)]";
const README_LINK_CLASS: &str = "mt-1 inline-flex w-fit items-center text-[15px] font-bold text-[var(--app-accent-strong)] no-underline hover:underline focus-visible:underline";

#[component]
pub fn Page01() -> Element {
    let data_load_request = use_context::<Signal<TemplateDataLoadRequest>>();
    let mut data_load_cache =
        use_context::<Signal<Option<Result<TemplateDataLoadResult, String>>>>();
    let initial_request_sequence = use_signal(|| data_load_request().sequence);
    let mut toast = use_context::<Signal<Option<Toast>>>();
    let mut toast_sequence = use_signal(|| 0_u64);
    let mut last_toast_key = use_signal(|| None::<String>);
    let data = use_resource(move || async move {
        let request = data_load_request();

        if request.sequence > initial_request_sequence() {
            refresh_template_data().await
        } else if let Some(cached) = data_load_cache.peek().clone() {
            cached
        } else {
            load_template_data().await
        }
    });

    use_effect(move || {
        if let Some(result) = data() {
            data_load_cache.set(Some(result));
        }
    });

    use_effect(move || {
        let request = data_load_request();
        let toast_details = match data() {
            Some(Ok(result)) => (
                format!("success:{}:{:?}", request.sequence, result.source),
                format!(
                    "{} {}",
                    template_data_source_label(&result.source),
                    t!("toast-action-loaded")
                ),
                ToastTone::Success,
            ),
            Some(Err(message)) => (
                format!("error:{}:{message}", request.sequence),
                format!("{} {}", t!("source-database"), t!("toast-action-error")),
                ToastTone::Error,
            ),
            None => (
                format!("loading:{}", request.sequence),
                format!("{} {}", t!("source-database"), t!("toast-action-loading")),
                ToastTone::Info,
            ),
        };

        if last_toast_key.peek().as_ref() == Some(&toast_details.0) {
            return;
        }

        let next_id = *toast_sequence.peek() + 1;
        toast_sequence.set(next_id);
        last_toast_key.set(Some(toast_details.0));
        toast.set(Some(Toast {
            id: next_id,
            message: toast_details.1,
            tone: toast_details.2,
        }));
    });

    let data_text = match data() {
        Some(Ok(result)) => format!("DB data is {}", result.data.message),
        Some(Err(message)) => message,
        None => t!("loading-template-data"),
    };

    rsx! {
        main { class: PAGE_CLASS,
            h1 { class: TITLE_CLASS, {t!("page-01-title")} }
            p { class: DATA_CLASS, "{data_text}" }
            div { class: BODY_CLASS,
                p { class: BODY_TEXT_CLASS, {t!("page-01-body-01")} }
                p { class: BODY_TEXT_CLASS, {t!("page-01-body-02")} }
                p { class: BODY_TEXT_CLASS,
                    {t!("page-01-body-03-prefix")}
                    " "
                    span { class: WARNING_CLASS, {t!("page-01-body-03-warning")} }
                }
            }
            a {
                class: README_LINK_CLASS,
                href: "https://github.com/SamuelAsherRivello/dioxus-project-template#readme",
                target: "_blank",
                rel: "noopener noreferrer",
                {t!("readme-details-link")}
            }
        }
    }
}

fn template_data_source_label(source: &TemplateDataSource) -> String {
    match source {
        TemplateDataSource::BrowserSnapshot | TemplateDataSource::Database => t!("source-database"),
    }
}
