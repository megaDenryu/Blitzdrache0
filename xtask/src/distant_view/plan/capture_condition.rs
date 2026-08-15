//! 採取1回ぶんの条件。担当するのは「どの旗を立ててどのアセットルートから採るか」を1つの値として持つことだけである。
//!
//! 真偽の欄が並ぶため、必須の名前だけを構築の口で受け、立てる旗をメソッドの連鎖で足す形にする。
//! 6つの真偽を1つの引数の並びへ書くと、同じ型の旗どうしを取り違えても署名が通る。

/// 採取が読むアセットルートの別。散布の検査点だけが対照側で別のルートを読む。
/// 散布の有無は焼き方の指定であり起動の旗ではないため、旗でなくルートで分かれる。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::distant_view) enum 読むアセットルート {
    計測入力,
    散布を焼かない対照,
}

pub(in crate::distant_view) struct 採取条件 {
    pub(in crate::distant_view) 名前: &'static str,
    pub(in crate::distant_view) ssaoを使わない: bool,
    pub(in crate::distant_view) 遠景影を使わない: bool,
    /// 光のにじみと明るさの圧縮を組まない構成で採る指定。主判定はこの対で近傍不変を課す。
    /// 参照: `_doc/設計/大規模世界の生成と遠景.md`第5段階の検査点4
    pub(in crate::distant_view) 後処理を使わない: bool,
    /// 本番の色の代わりに影の欠落計器の診断色を出す指定。影の検査点が色差の帰属に使う。
    /// 参照: `_doc/設計/大規模世界の生成と遠景.md`第5段階の検査点5
    pub(in crate::distant_view) 影可視度を可視化する: bool,
    /// 大規模世界でも距離区分を再配分せず実用分割のまま採る指定。影の検査点の対照側が使う。
    /// 対照を再配分前のコミットの作業ツリーから採ると、比べている差に再配分以外のコミットの差が混ざる。
    /// 同じバイナリで旗だけを分ければ、2枚の違いは再配分だけになる。
    pub(in crate::distant_view) 明示境界を使わない: bool,
    pub(in crate::distant_view) 読むルート: 読むアセットルート,
}

impl 採取条件 {
    pub(super) fn 名前から始める(名前: &'static str) -> Self {
        Self {
            名前,
            ssaoを使わない: false,
            遠景影を使わない: false,
            後処理を使わない: false,
            影可視度を可視化する: false,
            明示境界を使わない: false,
            読むルート: 読むアセットルート::計測入力,
        }
    }

    pub(super) fn ssaoを切る(mut self) -> Self {
        self.ssaoを使わない = true;
        self
    }

    pub(super) fn 遠景影を切る(mut self) -> Self {
        self.遠景影を使わない = true;
        self
    }

    pub(super) fn 後処理を切る(mut self) -> Self {
        self.後処理を使わない = true;
        self
    }

    pub(super) fn 影可視度の可視化を立てる(mut self) -> Self {
        self.影可視度を可視化する = true;
        self
    }

    pub(super) fn 明示境界を切る(mut self) -> Self {
        self.明示境界を使わない = true;
        self
    }

    pub(super) fn 散布を焼かない対照から読む(mut self) -> Self {
        self.読むルート = 読むアセットルート::散布を焼かない対照;
        self
    }
}
