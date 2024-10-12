/// The `Query` trait provides methods for building CSS selectors based on ARIA roles
/// and for querying elements by role, text, or test ID.
pub trait Query {
    /// Builds a CSS selector string based on the provided ARIA role.
    ///
    /// # Arguments
    ///
    /// * `role` - A string slice that holds the ARIA role for which to build the CSS selector.
    ///
    /// # Returns
    ///
    /// A `String` containing the CSS selector corresponding to the provided ARIA role.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yewlish_testing_tools::Query;
    ///
    /// struct MyQuery;
    ///
    /// impl Query for MyQuery {
    ///     fn build_role_query(&self, role: &str) -> String {
    ///         match role {
    ///             "button" => "button, input[type='button'], input[type='image'], input[type='reset'], input[type='submit'], summary",
    ///             _ => "",
    ///         }.to_string()
    ///     }
    ///
    ///     fn query_by_role(&self, role: &str) -> Self {
    ///         // Implementation for querying by role
    ///         Self
    ///     }
    ///
    ///     fn query_by_text(&self, text: &str) -> Self {
    ///         // Implementation for querying by text
    ///         Self
    ///     }
    ///
    ///     fn query_by_testid(&self, testid: &str) -> Self {
    ///         // Implementation for querying by test ID
    ///         Self
    ///     }
    ///
    ///     fn query_all_by_role(&self, role: &str) -> Vec<Self> {
    ///         // Implementation for querying all by role
    ///         vec![Self]
    ///     }
    ///
    ///     fn query_all_by_text(&self, text: &str) -> Vec<Self> {
    ///         // Implementation for querying all by text
    ///         vec![Self]
    ///     }
    ///
    ///     fn query_all_by_testid(&self, testid: &str) -> Vec<Self> {
    ///         // Implementation for querying all by test ID
    ///         vec![Self]
    ///     }
    /// }
    ///
    /// let query = MyQuery;
    /// let selector = query.build_role_query("button");
    /// assert_eq!(
    ///     selector,
    ///     "button, input[type='button'], input[type='image'], input[type='reset'], input[type='submit'], summary"
    /// );
    /// ```
    fn build_role_query(&self, role: &str) -> String {
        match role {
            "link" => "a[href], area[href]",
            "generic" => {
                r"
                a:not([href]), area:not([href]), b, bdi, bdo, body, data, div,
                article footer, aside footer, main footer, nav footer, section footer,
                [role='article'] footer, [role='complementary'] footer, [role='main'] footer,
                [role='navigation'] footer, [role='region'] footer, article header,
                aside header, main header, nav header, section header, [role='article'] header,
                [role='complementary'] header, [role='main'] header, [role='navigation'] header,
                [role='region'] header, i, li:not(ul li):not(ol li):not(menu li), pre, q, samp,
                section:not([aria-label]):not([aria-labelledby]):not([title]), small, span, u"
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
            "textbox" => r"
                input:not([list]):not([type]), input[type='email']:not([list]), input[type='text']:not([list]),
                input[type='tel']:not([list]), input[type='url']:not([list]), textarea
            ",
            "combobox" => r"
                input[list]:not([type]), input[list][type='text'], input[list][type='search'],
                input[list][type='tel'], input[list][type='url'], input[list][type='email'],
                select:not([multiple]):not([size]), select[size='1']:not([multiple])
            ",
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
            "columnheader" | "rowheader" => "table th, [role='table'] th, [role='grid'] th, [role='treegrid'] th",
            "time" => "time",
            "row" => "tr",
            _ => "",
        }.to_string()
    }

    #[must_use]
    /// Queries an element by its ARIA role.
    ///
    /// # Arguments
    ///
    /// * `role` - A string slice that holds the ARIA role to query.
    ///
    /// # Returns
    ///
    /// An instance of the implementing type representing the queried element.
    ///
    /// # Examples
    fn query_by_role(&self, role: &str) -> Self;

    #[must_use]
    /// Queries an element by its text content.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text content to query.
    ///
    /// # Returns
    ///
    /// An instance of the implementing type representing the queried element.
    fn query_by_text(&self, text: &str) -> Self;

    #[must_use]
    /// Queries an element by its test ID.
    ///
    /// # Arguments
    ///
    /// * `testid` - A string slice that holds the test ID to query.
    ///
    /// # Returns
    ///
    /// An instance of the implementing type representing the queried element.
    ///
    /// # Examples
    fn query_by_testid(&self, testid: &str) -> Self;

    #[must_use]
    /// Queries an element by its test ID.
    ///
    /// # Arguments
    ///
    /// * `testid` - A string slice that holds the test ID to query.
    ///
    /// # Returns
    ///
    /// An instance of the implementing type representing the queried element.
    fn query_all_by_role(&self, role: &str) -> Vec<Self>
    where
        Self: std::marker::Sized;

    /// Queries all elements by their text content.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text content to query.
    ///
    /// # Returns
    ///
    /// A `Vec` containing instances of the implementing type representing the queried elements.
    fn query_all_by_text(&self, text: &str) -> Vec<Self>
    where
        Self: std::marker::Sized;

    /// Queries all elements by their test ID.
    ///
    /// # Arguments
    ///
    /// * `testid` - A string slice that holds the test ID to query.
    ///
    /// # Returns
    ///
    /// A `Vec` containing instances of the implementing type representing the queried elements.
    fn query_all_by_testid(&self, testid: &str) -> Vec<Self>
    where
        Self: std::marker::Sized;
}
