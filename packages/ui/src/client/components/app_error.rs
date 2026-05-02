use dioxus::prelude::*;

#[component]
pub fn AppErrorFallback(error_context: ErrorContext) -> Element {
    let error_message = error_context
        .error()
        .map(|error| error.to_string())
        .unwrap_or_else(|| "The app encountered an unexpected rendering error.".to_string());

    rsx! {
        main { class: "mx-auto w-[min(calc(100%_-_40px),1120px)] py-12 pb-[72px] max-md:w-[min(calc(100%_-_56px),1120px)] max-md:py-8 max-md:pb-14",
            section { class: "rounded-lg border border-[var(--app-negative)] bg-[var(--app-surface)] px-5 py-[18px] text-[var(--app-text)] shadow-[var(--app-shadow-soft)]",
                h1 { class: "mb-2 mt-0 text-2xl text-[var(--app-negative)]", "Something went wrong" }
                p { class: "mb-4 mt-0 text-[var(--app-text-muted)]", "{error_message}" }
                button {
                    class: "min-h-10 cursor-pointer rounded-lg border border-[var(--app-control-border)] bg-[var(--app-control)] px-3.5 py-2 font-bold text-[var(--app-text)] hover:bg-[var(--app-accent-soft)] hover:text-[var(--app-accent-strong)] focus-visible:bg-[var(--app-accent-soft)] focus-visible:text-[var(--app-accent-strong)]",
                    r#type: "button",
                    onclick: move |_| {
                        error_context.clear_errors();
                    },
                    "Try again"
                }
            }
        }
    }
}
