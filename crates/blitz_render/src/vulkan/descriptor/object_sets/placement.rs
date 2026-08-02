//! 束の中のディスクリプタセットの並びで、描画対象添字・材質スロット添字・フレームスロット添字を位置へ写す配置。
//! 添字を掛け合わせた1本の配列を扱う場所をここへ閉じるのは、添字を取り違えると別の材質を束ねたまま描画が成立してしまうためである。
//! 描画対象ごとにスロット数が違うため、対象の位置は掛け算では出せず、対象ごとの開始位置とスロット数の両方を持つ必要がある。
//! スロット数を持たないと、ある対象が持たないスロット添字が隣の対象のセットを指したまま範囲内になる。

use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

pub(super) struct セット配置 {
    /// 描画対象ごとの、材質スロット添字0が入る位置とその対象が持つスロット数。要素数は描画対象数である。
    対象別: Vec<(usize, usize)>,
    合計スロット数: usize,
}

impl セット配置 {
    pub(super) fn 生成する(対象別スロット数: &[usize]) -> Self {
        let mut 対象別 = Vec::with_capacity(対象別スロット数.len());
        let mut 累計 = 0usize;
        for スロット数 in 対象別スロット数 {
            対象別.push((累計, *スロット数));
            累計 = 累計
                .checked_add(*スロット数)
                .unwrap_or_else(|| panic!("材質スロットの合計数がusizeを超えた"));
        }
        Self {
            対象別,
            合計スロット数: 累計,
        }
    }

    pub(super) fn 描画対象数(&self) -> usize {
        self.対象別.len()
    }

    /// 束の全描画対象の材質スロットを足した数。材質のセットの数そのものである。
    pub(super) fn 材質セット数(&self) -> usize {
        self.合計スロット数
    }

    /// ジオメトリのセットの数。描画対象ごとに進行中フレーム数だけ持つ。
    pub(super) fn ジオメトリセット数(&self) -> usize {
        self.描画対象数()
            .checked_mul(進行中フレーム数)
            .unwrap_or_else(|| panic!("ジオメトリのセット数がusizeを超えた"))
    }

    /// 範囲外の描画対象添字には`None`を返す。
    pub(super) fn ジオメトリ位置(&self, 描画対象添字: usize, フレーム添字: フレームスロット添字) -> Option<usize> {
        if 描画対象添字 >= self.描画対象数() {
            return None;
        }
        描画対象添字.checked_mul(進行中フレーム数)?.checked_add(フレーム添字.配列添字())
    }

    /// その対象が持たないスロット添字と範囲外の対象添字には`None`を返す。
    pub(super) fn 材質位置(&self, 描画対象添字: usize, スロット添字: usize) -> Option<usize> {
        let (開始, スロット数) = *self.対象別.get(描画対象添字)?;
        if スロット添字 >= スロット数 {
            return None;
        }
        開始.checked_add(スロット添字)
    }
}
