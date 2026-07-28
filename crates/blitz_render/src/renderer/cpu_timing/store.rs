//! ウォームアップ後の区間時間を貯める側。触れるのは標本の列と呼出回数だけであり、時刻の取得は行わない。
//! レンダラーの寿命いっぱい生き、終了時に一度だけ読まれる。1フレームで閉じる`clock`と呼ばれる頻度も寿命も違うため分けている。

use super::CPU区間時間;

pub(in crate::renderer) struct CPU区間計測 {
    ウォームアップフレーム数: u32,
    呼出回数: u32,
    時間一覧: Vec<CPU区間時間>,
}

impl CPU区間計測 {
    pub(in crate::renderer) fn 生成する(ウォームアップフレーム数: u32, 標本容量: usize) -> Self {
        Self {
            ウォームアップフレーム数,
            呼出回数: 0,
            時間一覧: Vec::with_capacity(標本容量),
        }
    }

    pub(in crate::renderer) fn 記録する(&mut self, 時間: CPU区間時間) {
        if self.呼出回数 >= self.ウォームアップフレーム数 {
            self.時間一覧.push(時間);
        }
        self.呼出回数 = self.呼出回数.saturating_add(1);
    }

    pub(in crate::renderer) fn 時間一覧(&self) -> &[CPU区間時間] {
        &self.時間一覧
    }
}
