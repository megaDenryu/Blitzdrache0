//! 検査結果: 概要と指摘一覧、および合否。担当するのは集計と判定であり、個々の検査項目は知らない。
//!
//! 合否を違反の件数だけで決め、警告では落とさない。Blenderの書き出しには作者が気にしていない宣言(既定で付く`doubleSided`など)が常に混ざり、
//! それで落とすと運用者が道具の終了コードを見なくなるためである。警告は件数を出して読ませることに徹する。

use super::finding::{契約指摘, 重大度};
use super::target::対象位置;

/// 検査した文書の規模。絵に出ない事故の多くが位置と大きさの取り違えであるため、境界箱を含める。
pub struct 契約検査概要 {
    pub(super) メッシュ数: usize,
    pub(super) プリミティブ数: usize,
    pub(super) 頂点数: usize,
    pub(super) インデックス数: usize,
    /// 先頭メッシュの頂点位置の最小と最大。頂点を1つも読めなかったときは`None`。
    pub(super) 境界箱: Option<([f32; 3], [f32; 3])>,
}

pub struct 契約検査結果 {
    概要: Option<契約検査概要>,
    指摘一覧: Vec<契約指摘>,
}

impl 契約検査結果 {
    pub(super) fn 生成する(概要: Option<契約検査概要>, 指摘一覧: Vec<契約指摘>) -> Self {
        Self { 概要, 指摘一覧 }
    }

    /// 文書そのものを開けなかったときの結果。概要を持たないことが、検査項目の走査に到達していないことを表す。
    pub(super) fn 開けなかった(理由: String) -> Self {
        Self {
            概要: None,
            指摘一覧: vec![契約指摘::違反を作る(
                対象位置::文書,
                format!("glTFとして開けなかった: {理由}"),
                "パスが実在するかを確かめ、.glbならBlenderの書き出しをやり直す。外部バッファ・外部画像を持つ.gltfは同じディレクトリに実体を置く",
            )],
        }
    }

    pub fn 概要(&self) -> Option<&契約検査概要> {
        self.概要.as_ref()
    }

    pub fn 指摘一覧(&self) -> &[契約指摘] {
        &self.指摘一覧
    }

    pub fn 違反件数(&self) -> usize {
        self.件数を数える(重大度::違反)
    }

    pub fn 警告件数(&self) -> usize {
        self.件数を数える(重大度::警告)
    }

    pub fn 合格か(&self) -> bool {
        self.違反件数() == 0
    }

    fn 件数を数える(&self, 対象: 重大度) -> usize {
        self.指摘一覧.iter().filter(|指摘| 指摘.重大度() == 対象).count()
    }
}
