use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};

const PROMPT_BACKDROP_CLASS: &str = "fixed inset-0 z-30 grid place-items-center bg-[var(--app-background)]/55 px-4 backdrop-blur-[14px]";
const PROMPT_DIALOG_CLASS: &str = "grid w-[min(360px,calc(100vw_-_32px))] gap-5 rounded-lg border border-[var(--app-border)] bg-[var(--app-surface-strong)] p-6 text-center shadow-[var(--app-shadow-soft)]";
const PROMPT_TITLE_CLASS: &str =
    "m-0 text-[24px] font-extrabold leading-tight text-[var(--app-text)]";
const PROMPT_DESCRIPTION_CLASS: &str = "sr-only";
const PROMPT_ACTIONS_CLASS: &str = "flex items-center justify-center gap-3";
const PROMPT_BUTTON_CLASS: &str = "inline-flex h-10 min-w-24 cursor-pointer items-center justify-center rounded-lg border border-[var(--app-control-border)] bg-[var(--app-control)] px-4 text-sm font-bold text-[var(--app-text-muted)] transition duration-150 hover:-translate-y-px hover:bg-[var(--app-control-hover)] hover:text-[var(--app-accent-strong)] focus-visible:-translate-y-px focus-visible:bg-[var(--app-control-hover)] focus-visible:text-[var(--app-accent-strong)]";
const PROMPT_PRIMARY_BUTTON_CLASS: &str = "inline-flex h-10 min-w-24 cursor-pointer items-center justify-center rounded-lg border border-[var(--app-accent)] bg-[var(--app-accent-soft)] px-4 text-sm font-bold text-[var(--app-accent-strong)] transition duration-150 hover:-translate-y-px hover:bg-[var(--app-control-hover)] focus-visible:-translate-y-px focus-visible:bg-[var(--app-control-hover)]";

#[component]
pub fn ConfirmationPrompt(mut open: Signal<bool>, on_answer: EventHandler<bool>) -> Element {
    rsx! {
        AlertDialogRoot {
            class: PROMPT_BACKDROP_CLASS,
            open: open(),
            on_open_change: move |value| open.set(value),
            AlertDialogContent {
                class: PROMPT_DIALOG_CLASS,
                AlertDialogTitle {
                    class: PROMPT_TITLE_CLASS,
                    "Are you sure?"
                }
                AlertDialogDescription {
                    class: PROMPT_DESCRIPTION_CLASS,
                    "Confirm or cancel this prompt."
                }
                AlertDialogActions { class: PROMPT_ACTIONS_CLASS,
                    AlertDialogAction {
                        class: PROMPT_PRIMARY_BUTTON_CLASS,
                        on_click: move |_| on_answer.call(true),
                        "Ok"
                    }
                    AlertDialogCancel {
                        class: PROMPT_BUTTON_CLASS,
                        on_click: move |_| on_answer.call(false),
                        "Cancel"
                    }
                }
            }
        }
    }
}
