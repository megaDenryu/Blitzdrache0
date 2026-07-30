//! カスケードの帯の識別子と、帯が受け持つビュー空間深度の区間。
//! 帯を`usize`で持ち回ると、範囲外の添字と「4帯である」という前提が呼び出し側に散るため、列挙で表す。

use blitz_math::メートル;

use super::帯数;

/// カメラからの距離帯。番号が大きいほどカメラから遠い。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 帯番号 {
    帯0,
    帯1,
    帯2,
    帯3,
}

impl 帯番号 {
    /// カメラに近い順の全帯。シャドウ記録も資源の生成もこの順で回す。
    pub fn 全帯() -> [Self; 帯数] {
        [Self::帯0, Self::帯1, Self::帯2, Self::帯3]
    }

    pub fn 添字(self) -> usize {
        match self {
            Self::帯0 => 0,
            Self::帯1 => 1,
            Self::帯2 => 2,
            Self::帯3 => 3,
        }
    }

    /// この帯のシャドウ記録がレンダーグラフへ積むパスの名前。GPU計測はパス名で移動平均を参照するため、
    /// 帯を跨いだ合算にせず帯ごとに固定の文字列を持つ。合計を出す側もこの名前で参照する。
    pub fn シャドウパス名(self) -> &'static str {
        match self {
            Self::帯0 => "シャドウ帯0",
            Self::帯1 => "シャドウ帯1",
            Self::帯2 => "シャドウ帯2",
            Self::帯3 => "シャドウ帯3",
        }
    }

    pub(crate) fn 次の帯(self) -> Self {
        match self {
            Self::帯0 => Self::帯1,
            Self::帯1 => Self::帯2,
            Self::帯2 | Self::帯3 => Self::帯3,
        }
    }
}

/// 1つの帯が受け持つカメラ視錐台のスライス。深度はカメラ前方向に沿ったビュー空間の距離である。
/// 隣の帯との重なりぶんだけ近側へ伸びており、`近深度`は1つ手前の帯の`遠深度`より小さい。
#[derive(Debug, Clone, Copy)]
pub struct 帯区間 {
    近深度: メートル,
    遠深度: メートル,
}

impl 帯区間 {
    pub(super) fn 生成する(近深度: メートル, 遠深度: メートル) -> Self {
        Self { 近深度, 遠深度 }
    }

    pub fn 近深度(self) -> メートル {
        self.近深度
    }

    pub fn 遠深度(self) -> メートル {
        self.遠深度
    }
}
