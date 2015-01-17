// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};
use Pos;

/// Point
///
/// [GeoJSON Format Specification § 2.1.2](http://geojson.org/geojson-spec.html#point)
#[derive(RustcEncodable, Clone)]
pub struct Point {
    pub coordinates: Pos,
}

impl ToJson for Point {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "Point".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}

#[test]
fn test_point_tojson() {
    let point = Point {coordinates: Pos(vec![1., 2., 3.])};
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"coordinates\":[1.0,2.0,3.0],\"type\":\"Point\"}", json_string);
}
