//! UIテクスチャ用ディスクリプタ(layout・pool・テクスチャごとのset)の生成手順を束ねる。

mod layout;
mod pool;
mod set;

pub(crate) use layout::生成する as layoutを生成する;
pub(crate) use pool::生成する as poolを生成する;
pub(crate) use set::{割り当てて書き込む, 解放する};
