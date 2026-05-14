use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub enum Locale {
    English,
    Japanese,
}

struct LocaleStore {
    english: HashMap<&'static str, &'static str>,
    japanese: HashMap<&'static str, &'static str>,
}

static STORE: OnceLock<LocaleStore> = OnceLock::new();

pub fn tr(locale: Locale, key: &str) -> &'static str {
    let store = STORE.get_or_init(|| LocaleStore {
        english: parse(include_str!("../locales/en-US.lang")),
        japanese: parse(include_str!("../locales/ja-JP.lang")),
    });

    let localized = match locale {
        Locale::English => store.english.get(key),
        Locale::Japanese => store.japanese.get(key),
    };

    localized
        .or_else(|| store.english.get(key))
        .copied()
        .unwrap_or("")
}

fn parse(data: &'static str) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        let key = leak(key.trim());
        let value = leak(unescape(value.trim()));
        map.insert(key, value);
    }
    map
}

fn unescape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn leak<T: Into<String>>(value: T) -> &'static str {
    Box::leak(value.into().into_boxed_str())
}
