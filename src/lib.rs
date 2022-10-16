#![no_std]

/// Returns a [`bool`] for whether the terminal supports Unicode.
pub fn is_unicode_supported() -> bool {
    const TERM: &str = env!("TERM");

    if !cfg!(windows) {
        return TERM != "linux"; // Linux console (kernel)
    }

    const CI: Option<&str> = option_env!("CI");
    const WT_SESSION: Option<&str> = option_env!("WT_SESSION");
    const TERMINUS_SUBLIME: Option<&str> = option_env!("TERMINUS_SUBLIME");
    const CON_EMU_TASK: Option<&str> = option_env!("ConEmuTask");
    const TERM_PROGRAM: Option<&str> = option_env!("TERM_PROGRAM");
    const TERMINAL_EMULATOR: Option<&str> = option_env!("TERMINAL_EMULATOR");

    CI.is_some()
        || WT_SESSION.is_some()
        || TERMINUS_SUBLIME.is_some()
        || TERM == "xterm-256color"
        || TERM == "alacritty"
        || if let Some(cet) = CON_EMU_TASK {
            cet == "{cmd::Cmder}"
        } else if let Some(tp) = TERM_PROGRAM {
            tp == "Terminus-Sublime" || tp == "vscode"
        } else if let Some(te) = TERMINAL_EMULATOR {
            te == "JetBrains-JediTerm"
        } else {
            false
        }
}
