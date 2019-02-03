use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn new_post(out: &mut Write, catalog: &Catalog) -> io::Result<()> {
out.write_all(b"<article class=\"media\"><!-- Begin new post -->\n    <figure class=\"media-left\">\n        <p class=\"image is-64x64\">\n        <img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"")?;
i18n!(catalog, "Username").to_html(out)?;
out.write_all(b"\">\n        </p>\n    </figure>\n    <div class=\"media-content\">\n        <div class=\"field\">\n            <p class=\"control\">\n            <textarea class=\"textarea\" placeholder=\"")?;
i18n!(catalog, "Add a comment...").to_html(out)?;
out.write_all(b"\"></textarea>\n            </p>\n        </div>\n        <nav class=\"level\">\n            <div class=\"level-left\">\n                <div class=\"level-item\">\n                    <a class=\"button is-info\">")?;
i18n!(catalog, "Awoo").to_html(out)?;
out.write_all(b"</a>\n                </div>\n            </div>\n            <div class=\"level-right\">\n                <div class=\"level-item\">\n                    <label class=\"checkbox\">\n                        <input type=\"checkbox\"> ")?;
i18n!(catalog, "Press enter to awoo").to_html(out)?;
out.write_all(b"\n                    </label>\n                </div>\n            </div>\n        </nav>\n    </div>\n</article><!-- End of new post -->\n")?;
Ok(())
}
