//! 正規表現の式をパースし、抽象構文木に変換
use std::{
    error::Error,
    fmt::{self, Display}
    mem::take
};
