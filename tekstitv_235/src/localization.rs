use indoc::formatdoc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Locale {
    Fi,
    En,
}

impl Locale {
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

    pub fn help_text(self, program_name: &str) -> String {
        match self {
            Self::Fi => formatdoc!(
                "
                {program_name} - NHL teksti-tv -hakija

                Kaytto:
                    {program_name} [valinnat]

                Valinnat:
                    --help, -h, -?, /?       Nayta tama ohje
                    --lang <fi|en>, -l <fi|en> Valitse kieli
                    --lang=fi|en             Valitse kieli
                    fi | en                  Pikavalinta kielelle
                "
            ),
            Self::En => formatdoc!(
                "
                {program_name} - NHL teletext fetcher

                Usage:
                    {program_name} [options]

                Options:
                    --help, -h, -?, /?       Show this help
                    --lang <fi|en>, -l <fi|en> Select language
                    --lang=fi|en             Select language
                    fi | en                  Language shortcut
                "
            ),
        }
    }
}
