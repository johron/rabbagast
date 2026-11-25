use chrono::Locale;
use std::env;
use sys_locale::get_locale;

fn parse_locale_str(raw: &str) -> Option<Locale> {
    let base = raw.split(['.', '@']).next()?;

    if base.is_empty() || base == "C" || base == "POSIX" {
        return None;
    }

    base.replace('-', "_").parse::<Locale>().ok()
}

pub fn get_chrono_locale() -> Locale {
    for var in ["LC_ALL", "LC_TIME", "LANG"] {
        if let Ok(val) = env::var(var) {
            if let Some(locale) = parse_locale_str(&val) {
                return locale;
            }
        }
    }

    get_locale()
        .as_deref()
        .and_then(parse_locale_str)
        .unwrap_or(Locale::en_US)
}
