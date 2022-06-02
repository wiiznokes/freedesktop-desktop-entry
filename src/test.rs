use std::path::Path;

use crate::DesktopEntry;

#[test]
fn test() {
    let path = Path::new("tests/org.mozilla.firefox.desktop");

    let locales = &["fr", "fr_FR.UTF-8"];

    if let Ok(entry) = DesktopEntry::from_path(path.to_path_buf(), locales) {
        let e = DesktopEntry::localized_entry(
            None,
            entry.groups.get("Desktop Entry"),
            "GenericName",
            &["fr"],
        )
        .unwrap();

        assert_eq!(e, "Navigateur Web");
    }
}