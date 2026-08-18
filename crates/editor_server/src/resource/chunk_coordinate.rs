//! チャンク座標の型契約。チャンクの構造・高さ格子・材質重みの識別に使い、
//! 編集コマンドの道路対象・建物操作でも操作対象チャンクを指すために使う。
//! ここが持つのは座標の値だけであり、ファイルパスへの導出は`storage::file_repository`が担う
//! (「役割の型は自分の配置を知る」はパスの型の責務であり、座標の型の責務ではない)。

use serde::{Deserialize, Serialize};

/// チャンク座標とは、大域世界の格子上で1チャンクを指す整数の組のことである。
/// 負のチャンク座標も許す(世界の中心から四方へ拡張しうるため)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct チャンク座標 {
    pub x: i32,
    pub z: i32,
}

impl チャンク座標 {
    pub fn 生成する(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// `x_z`の形式の文字列表現。ファイル保管庫がチャンクディレクトリ名として使うほか、
    /// ログ等でもこの座標を短く言い表すのに使える。形式を持つテキストであるため、
    /// 呼び出し側が`format!`で組み立てず、この型のメソッドの1箇所へ綴りを閉じる。
    pub fn アンダースコア区切り表記(&self) -> String {
        format!("{}_{}", self.x, self.z)
    }
}
