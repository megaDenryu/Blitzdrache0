//! パス別GPU時間の移動平均(60フレーム窓、判断30)。値オブジェクト。
//!
//! 注意: ミリ秒はf64で保持する。u64のtick差分をf32へ収める安全な変換経路が
//! 標準に無い(f64::from(u32)は可能だがf64→f32はasキャストのみで、
//! ワークスペースのas_conversions denyにより使えない)ため、f64のまま扱う
//! (公開APIの「時間はf32ミリ秒でよい」は許容であり必須ではない)。

use std::collections::VecDeque;

const 窓サイズ: usize = 60;

pub(crate) struct 移動平均 {
    値一覧: VecDeque<f64>,
    合計: f64,
}

impl 移動平均 {
    pub(crate) fn 新規() -> Self {
        Self {
            値一覧: VecDeque::with_capacity(窓サイズ),
            合計: 0.0,
        }
    }

    pub(crate) fn 追加する(&mut self, 値ミリ秒: f64) {
        self.値一覧.push_back(値ミリ秒);
        self.合計 += 値ミリ秒;
        if self.値一覧.len() > 窓サイズ
            && let Some(古い値) = self.値一覧.pop_front()
        {
            self.合計 -= 古い値;
        }
    }

    /// 現在の窓の平均ミリ秒。`追加する`を1回も呼んでいない状態では呼ばれない
    /// 前提(呼び出し元は初回追加と同時にエントリを作る)だが、0除算を避けるため
    /// 件数0では0.0を返す。
    pub(crate) fn 平均(&self) -> f64 {
        if self.値一覧.is_empty() {
            return 0.0;
        }
        let 件数u16 = u16::try_from(self.値一覧.len()).unwrap_or_else(|_| panic!("移動平均の窓サイズがu16に収まらない"));
        self.合計 / f64::from(件数u16)
    }
}
