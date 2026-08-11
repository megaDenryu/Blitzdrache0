//! 焼き方の指定: 材料が同じでも生成物を変えるコンパイルの指定。増分の鍵の見出しへ綴りとして入れるため、
//! 生成物を変えうる指定を漏れなくこの型へ集める。
//!
//! 指定を鍵に入れないと、同じ出力ルートへ指定を変えて焼いた実行が前の指定の生成物を据え置く。

use blitz_asset_compiler::テクスチャ格納方針;

#[derive(Debug, Clone, Copy)]
pub(crate) struct 焼き方の指定<'指定> {
    pub(crate) 世界名: &'指定 str,
    pub(crate) 同居植生個体数: usize,
    pub(crate) テクスチャ格納方針: テクスチャ格納方針,
}

impl 焼き方の指定<'_> {
    pub(super) fn 綴りを作る(self) -> String {
        format!(
            "世界={} 同居植生個体数={} テクスチャ格納方針={:?}",
            self.世界名, self.同居植生個体数, self.テクスチャ格納方針
        )
    }
}
