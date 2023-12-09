#[test]
fn test() {
    let text: String = "&amp; is an HTML entity but &ambassador; is not.".to_string();
    let out: String = "& is an HTML entity but &ambassador; is not.".to_string();
    assert_eq!(entity_parser(text), out);
}

// 投机取巧，直接用replace函数
pub fn entity_parser(text: String) -> String {
    use std::collections::HashMap;
    let map: HashMap<&str, &str> = HashMap::from([
        ("&quot;", "\""),
        ("&apos;", "\'"),
        ("&gt;", ">"),
        ("&lt;", "<"),
        ("&frasl;", "/"),
        ("&amp;", "&"), // 最后replace&
    ]);

    let mut text: String = text;
    for (k, v) in map {
        text = text.replace(k, v);
    }

    text
}
