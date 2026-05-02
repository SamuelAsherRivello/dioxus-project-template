use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::client::components::developer_tools::DeveloperTools;
use crate::client::components::toast::{Toast, ToastRegion};
use crate::client::Route;

const HEADER_CLASS: &str = "sticky top-0 z-10 mx-auto min-h-[84px] w-[min(1120px,calc(100%_-_40px))] rounded-b-lg border-b border-[var(--app-border)] bg-[var(--app-background)]/90 px-5 backdrop-blur-[18px] max-md:min-h-[68px] max-md:w-[min(calc(100%_-_28px),1120px)] max-md:px-3 max-md:py-[7px] max-sm:w-[min(calc(100%_-_16px),1120px)] max-sm:px-2";
const NAV_CLASS: &str = "flex min-h-[84px] w-full min-w-0 items-center justify-between gap-3 whitespace-nowrap max-md:min-h-[54px] max-md:gap-2 max-sm:gap-1";
const PAGE_LIST_CLASS: &str = "flex min-w-0 flex-1 items-center gap-2 max-md:gap-1 max-sm:gap-0.5";
const PAGE_LINK_BASE_CLASS: &str = "page-header__page-link inline-flex min-h-10 min-w-10 flex-[0_1_auto] items-center justify-center overflow-hidden rounded-lg px-[13px] py-2.5 text-[15px] font-bold text-[var(--app-text-muted)] no-underline transition-colors duration-150 hover:bg-[var(--app-accent-soft)] hover:text-[var(--app-accent-strong)] focus-visible:bg-[var(--app-accent-soft)] focus-visible:text-[var(--app-accent-strong)] visited:text-[var(--app-text-muted)] max-md:w-auto max-md:min-w-0 max-md:max-w-[46px] max-md:flex-1 max-md:basis-0 max-md:px-0 max-sm:max-w-[38px] max-sm:min-h-9 max-sm:text-[13px] max-[430px]:max-w-[34px] max-[430px]:min-h-[34px] max-[430px]:text-xs";
const PAGE_LINK_ACTIVE_CLASS: &str = "page-header__page-link inline-flex min-h-10 min-w-10 flex-[0_1_auto] items-center justify-center overflow-hidden rounded-lg bg-[color-mix(in_srgb,var(--app-accent-soft)_44%,transparent)] px-[13px] py-2.5 text-[15px] font-bold text-[color-mix(in_srgb,var(--app-accent-strong)_76%,var(--app-text-muted))] no-underline transition-colors duration-150 hover:bg-[var(--app-accent-soft)] hover:text-[var(--app-accent-strong)] focus-visible:bg-[var(--app-accent-soft)] focus-visible:text-[var(--app-accent-strong)] visited:text-[color-mix(in_srgb,var(--app-accent-strong)_76%,var(--app-text-muted))] max-md:w-auto max-md:min-w-0 max-md:max-w-[46px] max-md:flex-1 max-md:basis-0 max-md:px-0 max-sm:max-w-[38px] max-sm:min-h-9 max-sm:text-[13px] max-[430px]:max-w-[34px] max-[430px]:min-h-[34px] max-[430px]:text-xs";

#[component]
pub fn PageHeader() -> Element {
    let active_route = use_route::<Route>();
    let is_page01 = active_route == (Route::Page01 {});
    let is_page02 = active_route == (Route::Page02 {});
    let is_page03 = active_route == (Route::Page03 {});
    let toast = use_context::<Signal<Option<Toast>>>();

    rsx! {
        header { id: "page-header", class: HEADER_CLASS,
            nav { class: NAV_CLASS,
                div { class: PAGE_LIST_CLASS,
                    Link {
                        class: if is_page01 {
                            PAGE_LINK_ACTIVE_CLASS
                        } else {
                            PAGE_LINK_BASE_CLASS
                        },
                        to: Route::Page01 {},
                        aria_current: if is_page01 { "page" } else { "false" },
                        aria_label: t!("view-page-01"),
                        "data-tooltip": t!("view-page-01"),
                        span { class: "page-header__label-full max-md:hidden", {t!("nav-page-01")} }
                        span { class: "page-header__label-short hidden max-md:inline", {t!("nav-page-01-short")} }
                    }
                    Link {
                        class: if is_page02 {
                            PAGE_LINK_ACTIVE_CLASS
                        } else {
                            PAGE_LINK_BASE_CLASS
                        },
                        to: Route::Page02 {},
                        aria_current: if is_page02 { "page" } else { "false" },
                        aria_label: t!("view-page-02"),
                        "data-tooltip": t!("view-page-02"),
                        span { class: "page-header__label-full max-md:hidden", {t!("nav-page-02")} }
                        span { class: "page-header__label-short hidden max-md:inline", {t!("nav-page-02-short")} }
                    }
                    Link {
                        class: if is_page03 {
                            PAGE_LINK_ACTIVE_CLASS
                        } else {
                            PAGE_LINK_BASE_CLASS
                        },
                        to: Route::Page03 {},
                        aria_current: if is_page03 { "page" } else { "false" },
                        aria_label: t!("view-page-03"),
                        "data-tooltip": t!("view-page-03"),
                        span { class: "page-header__label-full max-md:hidden", {t!("nav-page-03")} }
                        span { class: "page-header__label-short hidden max-md:inline", {t!("nav-page-03-short")} }
                    }
                }
                DeveloperTools {}
            }
            ToastRegion { toast }
        }
    }
}
