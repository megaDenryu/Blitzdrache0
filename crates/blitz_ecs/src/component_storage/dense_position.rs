//! 密な列の位置: 疎な配列が持つ、密な列のどこにその個体の値があるかを指す添字。
//! `usize` でなく `u32` で持つのは、疎な配列が全スロットぶんの長さを型ごとに持つため、1枠の大きさが占有量に直接効くからである。

/// 疎な配列の1枠が指す、密な列の中の位置。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 密な列の位置(u32);

impl 密な列の位置 {
    pub(crate) fn 生成する(位置: usize) -> Self {
        let Ok(値) = u32::try_from(位置) else {
            panic!("密な列の位置{位置}がu32の範囲を超えた(1つの型を持つ個体の数はスロットの総数を超えない)")
        };
        Self(値)
    }

    pub(crate) fn 列の添字として読む(self) -> usize {
        let Ok(添字) = usize::try_from(self.0) else {
            panic!("u32の密な列の位置がusizeへ収まらない(32ビット以上の環境だけを対象にする)")
        };
        添字
    }
}
