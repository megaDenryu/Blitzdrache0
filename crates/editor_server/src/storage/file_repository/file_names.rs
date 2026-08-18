//! 保存先のファイル名の綴りの正本。`world`・`chunk`の両モジュールが共有する
//! (綴りを2箇所へ書くとconformの重複検査が拒む)。

pub(super) const 構造ファイル名: &str = "構造.json";
pub(super) const 高さ格子ファイル名: &str = "高さ格子.f32raw";
pub(super) const 材質重みファイル名: &str = "材質重み.u8raw";
