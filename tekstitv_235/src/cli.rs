use std::path::Path;

use crate::localization::Locale;

#[derive(Clone, Copy, Debug)]
pub struct CliOptions {
    pub locale: Locale,
    pub show_help: bool,
}

pub fn parse_args(args: &[String]) -> CliOptions {
    CliOptions {
        locale: parse_locale(args),
        show_help: should_print_help(args),
    }
}

fn parse_locale(args: &[String]) -> Locale {
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if let Some(value) = arg.strip_prefix("--lang=") {
            return parse_locale_code(value).unwrap_or(Locale::Fi);
        }

        if arg == "--lang" || arg == "-l" {
            return args
                .next()
                .and_then(|value| parse_locale_code(value))
                .unwrap_or(Locale::Fi);
        }

        if let Some(locale) = parse_locale_code(arg) {
            return locale;
        }
    }

    Locale::Fi
}

fn parse_locale_code(value: &str) -> Option<Locale> {
    match value.to_ascii_lowercase().as_str() {
        "fi" | "finnish" => Some(Locale::Fi),
        "en" | "english" => Some(Locale::En),
        _ => None,
    }
}

pub fn maybe_print_help(options: CliOptions) -> bool {
    if !options.show_help {
        return false;
    }

    let program_name = resolve_program_name(std::env::args().next().as_deref());
    print_help(options.locale, &program_name);
    true
}

fn should_print_help(args: &[String]) -> bool {
    args.iter()
        .any(|arg| matches!(arg.as_str(), "--help" | "-h" | "-?" | "/?"))
}

fn resolve_program_name(arg0: Option<&str>) -> String {
    arg0.and_then(|value| Path::new(value).file_name())
        .and_then(|value| value.to_str())
        .unwrap_or(env!("CARGO_PKG_NAME"))
        .to_string()
}

fn print_help(locale: Locale, program_name: &str) {
    println!("{}", locale.help_text(program_name));
}
