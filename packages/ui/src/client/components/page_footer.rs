use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn PageFooter() -> Element {
    rsx! {
        footer { class: "mx-auto w-[min(1120px,calc(100%_-_40px))] px-5 pb-7 pt-[22px] text-center text-[13px] leading-[1.45] text-[var(--app-text-subtle)] max-md:w-[min(calc(100%_-_56px),1120px)] max-md:pb-6 max-md:pt-[18px]",
            {t!("footer-rights")}
        }
    }
}
