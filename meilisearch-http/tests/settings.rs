use std::time::Duration;
use std::convert::Into;

use async_std::task::{block_on, sleep};
use async_std::io::prelude::*;
use http_service::Body;
use serde_json::json;
use serde_json::Value;
use assert_json_diff::assert_json_eq;

mod common;

#[test]
fn write_all_and_delete() {
    let mut server = common::setup_server().unwrap();

    // 1 - Create the index

    let body = json!({
        "uid": "movies",
    }).to_string().into_bytes();

    let req = http::Request::post("/indexes").body(Body::from(body)).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 201);

    // 2 - Send the settings

    let json = json!({
        "rankingRules": [
            "_typo",
            "_words",
            "_proximity",
            "_attribute",
            "_words_position",
            "_exact",
            "dsc(release_date)",
            "dsc(rank)",
        ],
        "rankingDistinct": "movie_id",
        "attributeIdentifier": "uid",
        "attributesSearchable": [
            "uid",
            "movie_id",
            "title",
            "description",
            "poster",
            "release_date",
            "rank",
        ],
        "attributesDisplayed": [
            "title",
            "description",
            "poster",
            "release_date",
            "rank",
        ],
        "stopWords": [
            "the",
            "a",
            "an",
        ],
        "synonyms": {
            "wolverine": ["xmen", "logan"],
            "logan": ["wolverine"],
        }
    });

    let body = json.to_string().into_bytes();

    let req = http::Request::post("/indexes/movies/settings").body(Body::from(body)).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 202);

    block_on(sleep(Duration::from_secs(1)));

    // 3 - Get all settings and compare to the previous one

    let req = http::Request::get("/indexes/movies/settings").body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 200);

    let mut buf = Vec::new();
    block_on(res.into_body().read_to_end(&mut buf)).unwrap();
    let res_value: Value = serde_json::from_slice(&buf).unwrap();

    assert_json_eq!(json, res_value, ordered: false);

    // 4 - Delete all settings

    let req = http::Request::delete("/indexes/movies/settings").body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 202);

    block_on(sleep(Duration::from_secs(1)));

    // 5 - Get all settings and check if they are empty

    let req = http::Request::get("/indexes/movies/settings").body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 200);

    let mut buf = Vec::new();
    block_on(res.into_body().read_to_end(&mut buf)).unwrap();
    let res_value: Value = serde_json::from_slice(&buf).unwrap();

    let json = json!({
        "rankingRules": null,
        "rankingDistinct": null,
        "attributeIdentifier": null,
        "attributesSearchable": null,
        "attributesDisplayed": null,
        "stopWords": null,
        "synonyms": null,
    });

    assert_json_eq!(json, res_value, ordered: false);
}


#[test]
fn write_all_and_update() {
    let mut server = common::setup_server().unwrap();

    // 1 - Create the index

    let body = json!({
        "uid": "movies",
    }).to_string().into_bytes();

    let req = http::Request::post("/indexes").body(Body::from(body)).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 201);

    // 2 - Send the settings

    let json = json!({
        "rankingRules": [
            "_typo",
            "_words",
            "_proximity",
            "_attribute",
            "_words_position",
            "_exact",
            "dsc(release_date)",
            "dsc(rank)",
        ],
        "rankingDistinct": "movie_id",
        "attributeIdentifier": "uid",
        "attributesSearchable": [
            "uid",
            "movie_id",
            "title",
            "description",
            "poster",
            "release_date",
            "rank",
        ],
        "attributesDisplayed": [
            "title",
            "description",
            "poster",
            "release_date",
            "rank",
        ],
        "stopWords": [
            "the",
            "a",
            "an",
        ],
        "synonyms": {
            "wolverine": ["xmen", "logan"],
            "logan": ["wolverine"],
        }
    });

    let body = json.to_string().into_bytes();

    let req = http::Request::post("/indexes/movies/settings").body(Body::from(body)).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 202);

    block_on(sleep(Duration::from_secs(1)));

    // 3 - Get all settings and compare to the previous one

    let req = http::Request::get("/indexes/movies/settings").body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 200);

    let mut buf = Vec::new();
    block_on(res.into_body().read_to_end(&mut buf)).unwrap();
    let res_value: Value = serde_json::from_slice(&buf).unwrap();

    assert_json_eq!(json, res_value, ordered: false);

    // 4 - Update all settings

    let json_update = json!({
        "rankingRules": [
            "_typo",
            "_words",
            "_proximity",
            "_attribute",
            "_words_position",
            "_exact",
            "dsc(release_date)",
        ],
        "attributeIdentifier": "uid",
        "attributesSearchable": [
            "title",
            "description",
            "uid",
        ],
        "attributesDisplayed": [
            "title",
            "description",
            "release_date",
            "rank",
            "poster",
        ],
        "stopWords": [
        ],
        "synonyms": {
            "wolverine": ["xmen", "logan"],
            "logan": ["wolverine", "xmen"],
        }
    });

    let body_update = json_update.to_string().into_bytes();

    let req = http::Request::post("/indexes/movies/settings").body(Body::from(body_update)).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 202);

    block_on(sleep(Duration::from_secs(1)));

    // 5 - Get all settings and check if the content is the same of (4)

    let req = http::Request::get("/indexes/movies/settings").body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 200);

    let mut buf = Vec::new();
    block_on(res.into_body().read_to_end(&mut buf)).unwrap();
    let res_value: Value = serde_json::from_slice(&buf).unwrap();

    let res_expected = json!({
        "rankingRules": [
            "_typo",
            "_words",
            "_proximity",
            "_attribute",
            "_words_position",
            "_exact",
            "dsc(release_date)",
        ],
        "rankingDistinct": null,
        "attributeIdentifier": "uid",
        "attributesSearchable": [
            "title",
            "description",
            "uid",
        ],
        "attributesDisplayed": [
            "title",
            "description",
            "release_date",
            "rank",
            "poster",
        ],
        "stopWords": null,
        "synonyms": {
            "wolverine": ["xmen", "logan"],
            "logan": ["wolverine", "xmen"],
        }
    });

    assert_json_eq!(res_expected, res_value, ordered: false);
}
