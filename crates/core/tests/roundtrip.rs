use svgo_core::{parse, stringify, StringifyOptions};

#[test]
fn roundtrip_basic_svg() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><!-- c --><g fill="red"><path d="M0 0h10"/></g><text>hi</text></svg>"#;
    let doc = parse(input).expect("parse failed");
    let output = stringify(&doc, &StringifyOptions::default());
    assert_eq!(output, input);
}

#[test]
fn roundtrip_doctype() {
    let input = r#"<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"><svg xmlns="http://www.w3.org/2000/svg"></svg>"#;
    let doc = parse(input).expect("parse failed");
    let output = stringify(&doc, &StringifyOptions::default());
    assert_eq!(output, input);
}

#[test]
fn roundtrip_cdata() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><style><![CDATA[.foo { color: red; }]]></style></svg>"#;
    let doc = parse(input).expect("parse failed");
    let output = stringify(&doc, &StringifyOptions::default());
    assert_eq!(output, input);
}

#[test]
fn roundtrip_processing_instruction() {
    let input =
        r#"<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg"></svg>"#;
    let doc = parse(input).expect("parse failed");
    let output = stringify(&doc, &StringifyOptions::default());
    assert_eq!(output, input);
}

#[test]
fn roundtrip_entity_escaping_attrs() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><text title="&amp;&lt;&gt;&quot;">x</text></svg>"#;
    let doc = parse(input).expect("parse failed");
    let output = stringify(&doc, &StringifyOptions::default());
    assert_eq!(output, input);
}

#[test]
fn roundtrip_entity_escaping_text() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><text>&amp;&lt;&gt;</text></svg>"#;
    let doc = parse(input).expect("parse failed");
    let output = stringify(&doc, &StringifyOptions::default());
    assert_eq!(output, input);
}
