pub trait Query {
    fn build_role_query(&self, role: &str) -> String {
        match role {
            "link" => "a[href], area[href]",
            "generic" => {
                r#"
                a:not([href]), area:not([href]), b, bdi, bdo, body, data, div,
                article footer, aside footer, main footer, nav footer, section footer,
                [role='article'] footer, [role='complementary'] footer, [role='main'] footer,
                [role='navigation'] footer, [role='region'] footer, article header,
                aside header, main header, nav header, section header, [role='article'] header,
                [role='complementary'] header, [role='main'] header, [role='navigation'] header,
                [role='region'] header, i, li:not(ul li):not(ol li):not(menu li), pre, q, samp,
                section:not([aria-label]):not([aria-labelledby]):not([title]), small, span, u   
            "#
            }
            "group" => "address, details, fieldset, hgroup, optgroup",
            "article" => "article",
            "complementary" => "aside",
            "blockquote" => "blockquote",
            "button" => "button, input[type='button'], input[type='image'], input[type='reset'], input[type='submit'], summary",
            "caption" => "caption",
            "code" => "code",
            "listbox" => "datalist, select[multiple], select[size]:not([size='1'])",
            "deletion" => "del, s",
            "term" => "dfn",
            "dialog" => "dialog",
            "emphasis" => "em",
            "figure" => "figure",
            "contentinfo" => "footer:not(article footer):not(aside footer):not(main footer):not(nav footer):not(section footer):not([role='article'] footer):not([role='complementary'] footer):not([role='main'] footer):not([role='navigation'] footer):not([role='region'] footer)",
            "form" => "form",
            "heading" => "h1, h2, h3, h4, h5, h6",
            "banner" => "header:not(article header):not(aside header):not(main header):not(nav header):not(section header):not([role='article'] header):not([role='complementary'] header):not([role='main'] header):not([role='navigation'] header):not([role='region'] header)",
            "separator" => "hr",
            "document" => "html",
            "checkbox" => "input[type='checkbox']",
            "textbox" => r#"
                input:not([list]):not([type]), input[type='email']:not([list]), input[type='text']:not([list]),
                input[type='tel']:not([list]), input[type='url']:not([list]), textarea
            "#,
            "combobox" => r#"
                input[list]:not([type]), input[list][type='text'], input[list][type='search'],
                input[list][type='tel'], input[list][type='url'], input[list][type='email'],
                select:not([multiple]):not([size]), select[size='1']:not([multiple])
            "#,
            "spinbutton" => "input[type='number']",
            "radio" => "input[type='radio']",
            "slider" => "input[type='range']",
            "searchbox" => "input[type='search']:not([list])",
            "insertion" => "ins",
            "listitem" => "ul li, ol li, menu li",
            "menu" => "menu",
            "math" => "math",
            "list" => "ol, ul",
            "meter" => "meter",
            "navigation" => "nav",
            "option" => "select option, datalist option",
            "status" => "output",
            "paragraph" => "p",
            "progressbar" => "progress",
            "search" => "search",
            "region" => "section[aria-label], section[aria-labelledby], section[title]",
            "strong" => "strong",
            "subscript" => "sub",
            "graphics-document" => "svg",
            "table" => "table",
            "rowgroup" => "tbody, thead, tfoot",
            "cell" => "table td, [role='table'] td, table th, [role='table'] th",
            "gridcell" => "[role='grid'] td, [role='treegrid'] td, [role='grid'] th, [role='treegrid'] th",
            "columnheader" => "table th, [role='table'] th, [role='grid'] th, [role='treegrid'] th", 
            "rowheader" => "table th, [role='table'] th, [role='grid'] th, [role='treegrid'] th", 
            "time" => "time",
            "row" => "tr",
            _ => "",
        }
        .to_string()
    }

    fn query_by_role(&self, role: &str) -> Self;
    fn query_by_text(&self, text: &str) -> Self;
    fn query_by_testid(&self, testid: &str) -> Self;

    fn query_all_by_role(&self, role: &str) -> Vec<Self>
    where
        Self: std::marker::Sized;

    fn query_all_by_text(&self, text: &str) -> Vec<Self>
    where
        Self: std::marker::Sized;

    fn query_all_by_testid(&self, testid: &str) -> Vec<Self>
    where
        Self: std::marker::Sized;
}
