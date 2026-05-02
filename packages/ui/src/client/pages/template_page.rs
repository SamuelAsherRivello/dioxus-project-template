use dioxus::prelude::*;

const PAGE_CLASS: &str = "mx-auto min-w-0 w-[min(1120px,calc(100%_-_40px))] py-[42px] pb-14 max-md:w-[min(calc(100%_-_56px),1120px)] max-md:py-[30px] max-md:pb-[42px]";
const TITLE_CLASS: &str = "mb-[18px] mt-0 text-[clamp(36px,6vw,64px)] leading-none tracking-normal text-[var(--app-text)] max-md:text-[clamp(34px,12vw,54px)]";
const DATA_CLASS: &str = "mb-6 inline-flex min-h-10 items-center rounded-lg border border-[var(--app-border)] bg-[var(--app-surface)] px-3.5 py-2 text-base font-bold text-[var(--app-accent-strong)] shadow-[var(--app-shadow-soft)]";
const BODY_CLASS: &str = "grid min-w-0 w-full gap-[18px]";
const BODY_TEXT_CLASS: &str = "m-0 min-w-0 max-w-full break-words text-[19px] leading-[1.65] text-[var(--app-text-muted)] max-md:max-w-[30ch] max-md:text-[17px] max-md:leading-[1.58]";
const README_LINK_CLASS: &str = "mt-1 inline-flex w-fit items-center text-[15px] font-bold text-[var(--app-accent-strong)] no-underline hover:underline focus-visible:underline";

#[component]
pub fn TemplatePage(
    title: String,
    body_01: String,
    body_02: String,
    body_03: String,
    data_text: Option<String>,
    #[props(default = None)] readme_text: Option<String>,
    #[props(default = None)] readme_href: Option<String>,
) -> Element {
    rsx! {
        main { class: PAGE_CLASS,
            h1 { class: TITLE_CLASS, "{title}" }
            if let Some(data_text) = data_text {
                p { class: DATA_CLASS, "{data_text}" }
            }
            div { class: BODY_CLASS,
                p { class: BODY_TEXT_CLASS, "{body_01}" }
                p { class: BODY_TEXT_CLASS, "{body_02}" }
                p { class: BODY_TEXT_CLASS, "{body_03}" }
            }
            if let (Some(readme_text), Some(readme_href)) = (readme_text, readme_href) {
                a {
                    class: README_LINK_CLASS,
                    href: readme_href,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "{readme_text}"
                }
            }
        }
    }
}
