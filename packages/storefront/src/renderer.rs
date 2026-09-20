//! Simple HTML renderer.
//!
//! Plain Rust string building -- no Liquid, no theme engine, no template
//! DSL, per the chapter's explicit constraint. This function is pure:
//! context in, HTML out, no I/O, no clock, no cache. That's what makes it
//! trivially testable and what keeps "rendering" clearly separate from
//! "loading data" and "owning freshness".

use crate::context::ProductPageContext;

pub fn render_product_page(ctx: &ProductPageContext) -> String {
    let description = ctx
        .description
        .as_deref()
        .map(|d| format!("<p class=\"description\">{}</p>", escape_html(d)))
        .unwrap_or_default();

    format!(
        "<!doctype html>\n\
         <html lang=\"en\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>{title}</title>\n\
         </head>\n\
         <body>\n\
         <main>\n\
         <h1 class=\"product-title\">{title}</h1>\n\
         <p class=\"price\">{price}</p>\n\
         <p class=\"availability\">{availability}</p>\n\
         {description}\n\
         </main>\n\
         </body>\n\
         </html>\n",
        title = escape_html(&ctx.title),
        price = escape_html(&ctx.price_display),
        availability = escape_html(&ctx.availability),
        description = description,
    )
}

/// Product titles and descriptions are user-supplied and get interpolated
/// straight into HTML, so they must be escaped -- otherwise a title like
/// `<script>` becomes executable markup on the storefront page.
fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_html_special_characters() {
        assert_eq!(
            escape_html("<script>alert('x')</script>"),
            "&lt;script&gt;alert(&#39;x&#39;)&lt;/script&gt;"
        );
        assert_eq!(escape_html("Tea & Coffee"), "Tea &amp; Coffee");
    }
}
