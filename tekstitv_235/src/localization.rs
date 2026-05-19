#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Locale {
    Fi,
    En,
}

impl Locale {
    pub fn from_args(args: impl IntoIterator<Item = String>) -> Self {
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            if let Some(value) = arg.strip_prefix("--lang=") {
                return Self::from_code(value).unwrap_or(Self::Fi);
            }

            if arg == "--lang" || arg == "-l" {
                return args
                    .next()
                    .as_deref()
                    .and_then(Self::from_code)
                    .unwrap_or(Self::Fi);
            }

            if let Some(locale) = Self::from_code(&arg) {
                return locale;
            }
        }

        Self::Fi
    }

    fn from_code(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "fi" | "finnish" => Some(Self::Fi),
            "en" | "english" => Some(Self::En),
            _ => None,
        }
    }

    pub fn tonight_heading(self) -> &'static str {
        match self {
            Self::Fi => "NHL TANAAN ",
            Self::En => "NHL TONIGHT ",
        }
    }

    pub fn tomorrow_heading(self) -> &'static str {
        match self {
            Self::Fi => "NHL HUOMENNA ",
            Self::En => "NHL TOMORROW ",
        }
    }

    pub fn loading_heading(self) -> &'static str {
        match self {
            Self::Fi => "NHL LADATAAN ",
            Self::En => "NHL LOADING ",
        }
    }

    pub fn selection_heading(self) -> &'static str {
        match self {
            Self::Fi => "  SEURAAVA> ",
            Self::En => "    NEXT>    ",
        }
    }

    pub fn next_target_label(self) -> &'static str {
        match self {
            Self::Fi => "> Seuraava kohde:",
            Self::En => "> Next target:",
        }
    }

    pub fn no_active_game_message(self) -> &'static str {
        match self {
            Self::Fi => "Huoltotauko...",
            Self::En => "Maintenance break...",
        }
    }
}
