use std::io::Read;
use test;

use super::super::*;

#[test]
fn header_passing() {
    test::set_handler("/header_passing", |req, _url| {
        assert!(req.has("X-Foo"));
        assert_eq!(req.get("X-Foo").unwrap(), "bar");
        test::make_stream(200, "OK", vec!["X-Bar: foo"], vec![])
    });
    let resp = get("test://host/header_passing").set("X-Foo", "bar").call();
    assert_eq!(*resp.status(), 200);
    assert!(resp.has("X-Bar"));
    assert_eq!(resp.get("X-Bar").unwrap(), "foo");
}

#[test]
fn repeat_non_x_header() {
    test::set_handler("/repeat_non_x_header", |req, _url| {
        assert!(req.has("Accept"));
        assert_eq!(req.get("Accept").unwrap(), "baz");
        test::make_stream(200, "OK", vec![], vec![])
    });
    let resp = get("test://host/repeat_non_x_header")
        .set("Accept", "bar")
        .set("Accept", "baz")
        .call();
    assert_eq!(*resp.status(), 200);
}

#[test]
fn repeat_x_header() {
    test::set_handler("/repeat_x_header", |req, _url| {
        assert!(req.has("X-Forwarded-For"));
        assert_eq!(req.get("X-Forwarded-For").unwrap(), "130.240.19.2");
        assert_eq!(req.get_all("X-Forwarded-For"), vec![
            "130.240.19.2",
            "130.240.19.3",
        ]);
        test::make_stream(200, "OK", vec![], vec![])
    });
    let resp = get("test://host/repeat_x_header")
        .set("X-Forwarded-For", "130.240.19.2")
        .set("X-Forwarded-For", "130.240.19.3")
        .call();
    assert_eq!(*resp.status(), 200);
}

#[test]
fn body_as_text() {
    test::set_handler("/body_as_text", |_req, _url| {
        test::make_stream(200, "OK", vec![], "Hello World!".to_string().into_bytes())
    });
    let resp = get("test://host/body_as_text").call();
    let text = resp.into_string().unwrap();
    assert_eq!(text, "Hello World!");
}

#[test]
fn body_as_json() {
    test::set_handler("/body_as_json", |_req, _url| {
        test::make_stream(
            200,
            "OK",
            vec![],
            "{\"hello\":\"world\"}".to_string().into_bytes(),
        )
    });
    let resp = get("test://host/body_as_json").call();
    let json = resp.into_json().unwrap();
    assert_eq!(json["hello"], "world");
}

#[test]
fn body_as_reader() {
    test::set_handler("/body_as_reader", |_req, _url| {
        test::make_stream(200, "OK", vec![], "abcdefgh".to_string().into_bytes())
    });
    let resp = get("test://host/body_as_reader").call();
    let mut reader = resp.into_reader();
    let mut text = String::new();
    reader.read_to_string(&mut text).unwrap();
    assert_eq!(text, "abcdefgh");
}
