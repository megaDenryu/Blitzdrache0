//! 生成台帳: 大規模世界の生成が、チャンクごとの増分の鍵とソースの内容のハッシュを覚えておく機械が読む台帳。
//! 鍵が一致するチャンクを焼き直さないために置く。保存場所は生成の出力ルートの直下であり、カタログへは載せない。
//!
//! 増分の鍵は、チャンクごとに変わらない部分(乱数の種・生成器の版・生成器の実行ファイルの内容ハッシュ)を`heading`が、
//! チャンクごとに変わる部分(座標と内容ハッシュ)を`ledger`が持つ。畳み方は`content_hash`、テキスト形式は`text_format`、
//! 増分が効いたかの数え上げは`rebake_tally`、材料が複数あるときの畳み方は`dependency_digest`にある。
//! 参照: `_doc/計画/ユビキタス言語.md`、`_doc/設計/大規模世界の生成と遠景.md`

mod content_hash;
mod dependency_digest;
mod error;
#[cfg(test)]
mod generation_ledger_tests;
mod generator_version;
mod heading;
mod ledger;
mod map_seed;
mod rebake_tally;
mod text_format;

pub use content_hash::内容ハッシュ;
pub use dependency_digest::ソース依存一式の内容ハッシュを求める;
pub use error::生成台帳エラー;
pub use heading::生成台帳の見出し;
pub use ledger::{チャンクの焼き直し判定, 生成台帳};
pub use map_seed::{マップ生成の乱数の種, 種の由来};
pub use rebake_tally::焼き直しの勘定;
