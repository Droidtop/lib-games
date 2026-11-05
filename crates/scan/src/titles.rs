//! The one title key every device gives a game with no store id
//! (Droidtop/tracker#870; the owner, 2026-11-05: "letters and digits only,
//! but with space-to-hyphen conversion. Lutris's standard"): `title2:` and the
//! title's Lutris slug. "Being a DIK" is `title2:being-a-dik`.
//!
//! droidtop's `TitleKeys.slug` is the same function, so a game keyed on the
//! computer and on a handheld meets. The older key, `title:` and the title's
//! letters and digits joined by single spaces (sync-core's `title_key`), stays
//! a game's alias, so what was stored under it is linked to the new key.

use unicode_normalization::UnicodeNormalization;

use crate::Found;

/// The prefix of the title key.
pub const TITLE2: &str = "title2:";

/// Python's `\s` and `str.strip()` on an ASCII string: space, `\t` to `\r`,
/// and the separators `\x1c` to `\x1f`.
fn py_space(c: char) -> bool {
    matches!(c, ' ' | '\t'..='\r' | '\x1c'..='\x1f')
}

/// Lutris's `slugify` (lutris/lutris `lutris/util/strings.py` at
/// 4d9939e468de, GPL-3.0), re-implemented from its behaviour:
/// - NFD, then everything outside ASCII is dropped;
/// - every character but letters, digits, `_`, whitespace and `-` is removed;
/// - the ends are stripped of whitespace, and it is lower-cased;
/// - each run of hyphens and whitespace becomes one hyphen.
///
/// A leading or trailing hyphen stays (`"  -- Hello -- World"` is
/// `-hello-world`), as in Lutris. A title with nothing left (no Latin letters
/// or digits) is the UUID-5 of the title as given, in the URL namespace.
pub fn slug(title: &str) -> String {
    let kept: String =
        title.nfd().filter(|c| c.is_ascii() && (c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || py_space(*c))).collect();
    let mut out = String::with_capacity(kept.len());
    let mut in_run = false;
    for c in kept.trim_matches(py_space).chars() {
        if c == '-' || py_space(c) {
            if !in_run {
                out.push('-');
            }
            in_run = true;
        } else {
            out.push(c.to_ascii_lowercase());
            in_run = false;
        }
    }
    if out.is_empty() {
        return uuid5_url(title);
    }
    out
}

/// RFC 4122's version 5 UUID of `name` in the URL namespace
/// (`6ba7b811-9dad-11d1-80b4-00c04fd430c8`), as Python's
/// `str(uuid.uuid5(uuid.NAMESPACE_URL, name))`.
fn uuid5_url(name: &str) -> String {
    const NAMESPACE_URL: [u8; 16] = [0x6b, 0xa7, 0xb8, 0x11, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8];
    let mut sha = sha1_smol::Sha1::new();
    sha.update(&NAMESPACE_URL);
    sha.update(name.as_bytes());
    let mut b = sha.digest().bytes();
    b[6] = (b[6] & 0x0f) | 0x50;
    b[8] = (b[8] & 0x3f) | 0x80;
    let hex: String = b[..16].iter().map(|x| format!("{x:02x}")).collect();
    format!("{}-{}-{}-{}-{}", &hex[0..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..32])
}

/// The key of a game with no store id: `title2:` and [`slug`].
pub fn title_key(title: &str) -> String {
    format!("{TITLE2}{}", slug(title))
}

/// A game the scan keyed by the older `title:` key, re-keyed: its key becomes
/// [`title_key`] of its title and the old key one of its aliases. Any other
/// key is left as it is.
pub fn rekey(found: &mut Found) {
    if !found.game.key.starts_with("title:") {
        return;
    }
    let new = title_key(&found.game.title);
    let old = std::mem::replace(&mut found.game.key, new);
    if !found.aliases.contains(&old) {
        found.aliases.push(old);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each title's slug from Lutris's own `slugify`, run in Python 3 on the
    /// source at 4d9939e468de.
    const LUTRIS: &[(&str, &str)] = &[
        ("Being a DIK", "being-a-dik"),
        ("Assassin's Creed\u{00ae} II", "assassins-creed-ii"),
        ("S.T.A.L.K.E.R.", "stalker"),
        ("foo_bar-baz", "foo_bar-baz"),
        ("  -- Hello -- World", "-hello-world"),
        ("Pok\u{00e9}mon Caf\u{00e9}", "pokemon-cafe"),
        ("Half-Life 2: Episode One", "half-life-2-episode-one"),
        ("The Witcher 3: Wild Hunt \u{2013} GOTY", "the-witcher-3-wild-hunt-goty"),
        ("\u{30c9}\u{30ad}\u{30c9}\u{30ad}\u{6587}\u{82b8}\u{90e8}", "eb63f7d1-d5d1-571f-8642-44e41fd745dc"),
        ("Tab\tand\nnewline", "tab-and-newline"),
        ("Trailing - ", "trailing-"),
        ("\u{2122}", "62090136-1413-5465-ac9a-a32a05cf96e3"),
        ("Eternum 0.9.5", "eternum-095"),
        ("\u{00c6}on Flux", "on-flux"),
        ("Thief of Hearts", "thief-of-hearts"),
        ("ThiefofHearts", "thiefofhearts"),
    ];

    #[test]
    fn slug_is_lutris_slugify() {
        for (title, want) in LUTRIS {
            assert_eq!(slug(title), *want, "{title:?}");
        }
    }

    #[test]
    fn title_keyed_games_move_to_title2_and_keep_the_old_key() {
        let mut found = crate::pc_game("title:being a dik".into(), "Being a DIK".into(), None, "folder", 0, None);
        rekey(&mut found);
        assert_eq!(found.game.key, "title2:being-a-dik");
        assert_eq!(found.aliases, ["title:being a dik"]);

        let mut store = crate::pc_game("steam:1".into(), "Being a DIK".into(), None, "steam", 0, None);
        rekey(&mut store);
        assert_eq!(store.game.key, "steam:1");
        assert!(store.aliases.is_empty());
    }
}
