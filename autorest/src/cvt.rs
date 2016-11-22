// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use postgres::rows::Row;
use postgres::types::{FromSql, Type};
use serde_json::value::{Value, ToJson};
use serde::Serialize;

fn to_json<T>(row: &Row, idx: usize, is_nullable: bool) -> Value
    where T: ToJson + FromSql + Serialize {
    if is_nullable {
        let value: Option<T> = row.get(idx);
        value.to_json()
    } else {
        let value: T = row.get(idx);
        value.to_json()
    }
}

pub fn row_field_to_json_value(row: &Row, idx: usize, is_nullable: bool, ty: Type) -> Value {
    match ty {
        Type::Bool => to_json::<bool>(row, idx, is_nullable),
        Type::Char => to_json::<i8>(row, idx, is_nullable),
        Type::Int2 => to_json::<i16>(row, idx, is_nullable),
        Type::Int4 => to_json::<i32>(row, idx, is_nullable),
        Type::Int8 => to_json::<i64>(row, idx, is_nullable),
        Type::Float4 => to_json::<f32>(row, idx, is_nullable),
        Type::Float8 => to_json::<f64>(row, idx, is_nullable),
        Type::Varchar => to_json::<String>(row, idx, is_nullable),
        Type::Text => to_json::<String>(row, idx, is_nullable),
        _ => unimplemented!()
    }
}
