//! アセットが持つ材質の安定した識別子。担当するのは、永続する描画束が保持してよい唯一の材質の指し方を型にすることである。
//!
//! 注意: GPUのレコード添字を描画束が保持してはならない。添字は資源表世代ごとに決まり直すため、世代を作り直すと
//! 旧い添字が新しい表の別の材質を無言で指す。束はこのIDを持ち、フレームの描画準備がその時点で束縛する世代に対して解決する。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct 大域材質ID {
    値: u64,
}

impl 大域材質ID {
    pub(crate) const fn 生成する(値: u64) -> Self {
        Self { 値 }
    }

    /// 失敗の表示と台帳の鍵で使う生値。
    pub(crate) const fn 値(self) -> u64 {
        self.値
    }
}
