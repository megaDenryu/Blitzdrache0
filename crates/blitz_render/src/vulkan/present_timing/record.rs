//! 提示IDの発番と、表示観測の蓄積。どのIDを待つかの規律もここに閉じる。

use std::time::Instant;

use crate::present_display_observation::実表示観測;

/// 待機対象を「最後に発番したIDより1つ古い提示」に保つための戻り数。
///
/// 注意: これから提示するIDをNとすると待機対象はN-2であり、提示済みで未表示の列が2件以上たまっている
/// ときにしか停止しない。この遅れが、計測が描画ループのフレームペーシングを変えないための根拠である。
const 待機対象の戻り数: u64 = 1;

/// 待機の結末。間隔を持てるかどうかが異なるため、観測へ変換する前の中間表現として分ける。
pub(crate) enum 待機結末 {
    表示された { 停止時間ms: f64 },
    時間切れになった { 停止時間ms: f64 },
}

pub(crate) struct 表示時刻記録 {
    次に発番するid: u64,
    最後に発番したid: u64,
    未観測の先頭id: u64,
    前回観測時刻: Option<Instant>,
    ウォームアップ観測数: u32,
    観測回数: u32,
    観測一覧: Vec<実表示観測>,
}

impl 待機結末 {
    /// 間隔を持てない結末と、間隔が未確定な初回観測を`None`へ落とす。
    fn 観測にする(self, 間隔ms: Option<f64>) -> Option<実表示観測> {
        match self {
            Self::時間切れになった { 停止時間ms } => Some(実表示観測::待機が時間切れになった { 停止時間ms }),
            Self::表示された { 停止時間ms } => 間隔ms.map(|前回からの間隔ms| 実表示観測::表示を観測した {
                前回からの間隔ms,
                停止時間ms,
            }),
        }
    }
}

impl 表示時刻記録 {
    pub(crate) fn 生成する(ウォームアップ観測数: u32, 標本容量: usize) -> Self {
        Self {
            次に発番するid: 1,
            最後に発番したid: 0,
            未観測の先頭id: 1,
            前回観測時刻: None,
            ウォームアップ観測数,
            観測回数: 0,
            観測一覧: Vec::with_capacity(標本容量),
        }
    }

    pub(crate) fn 発番する(&mut self) -> u64 {
        let 発番id = self.次に発番するid;
        self.次に発番するid = 発番id.saturating_add(1);
        self.最後に発番したid = 発番id;
        発番id
    }

    /// このフレームで待つべき提示ID。提示がまだ足りていなければ`None`(待たない)。
    pub(crate) fn 待機対象id(&self) -> Option<u64> {
        let 待てる上限 = self.最後に発番したid.checked_sub(待機対象の戻り数)?;
        (self.未観測の先頭id <= 待てる上限).then_some(self.未観測の先頭id)
    }

    pub(crate) fn 観測を加える(&mut self, 結末: 待機結末, 完了時刻: Instant) {
        let 間隔ms = self.前回観測時刻.map(|前回| 完了時刻.duration_since(前回).as_secs_f64() * 1000.0);
        self.未観測の先頭id = self.未観測の先頭id.saturating_add(1);
        self.前回観測時刻 = Some(完了時刻);
        let Some(観測) = 結末.観測にする(間隔ms) else {
            return;
        };
        if self.観測回数 >= self.ウォームアップ観測数 {
            self.観測一覧.push(観測);
        }
        self.観測回数 = self.観測回数.saturating_add(1);
    }

    /// スワップチェーンを作り直したとき、古いスワップチェーンに属する提示の追跡を捨てる。
    pub(crate) fn 提示の追跡をやり直す(&mut self) {
        self.未観測の先頭id = self.次に発番するid;
        self.前回観測時刻 = None;
    }

    pub(crate) fn 観測一覧(&self) -> &[実表示観測] {
        &self.観測一覧
    }
}
