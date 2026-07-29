//! エンジンが宣言し、レンダラーが実行する1フレームの段階列。

use thiserror::Error;

mod validation;

use validation::検証する;

const 段階数上限: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum フレーム段階 {
    スキニング,
    布シミュレーション,
    影,
    シーン,
    空,
    粒子,
    ブルームとトーンマップ,
    UI,
    読み戻し,
}

impl フレーム段階 {
    pub fn 名称(self) -> &'static str {
        match self {
            Self::スキニング => "スキニング",
            Self::布シミュレーション => "布シミュレーション",
            Self::影 => "影",
            Self::シーン => "シーン",
            Self::空 => "空",
            Self::粒子 => "粒子",
            Self::ブルームとトーンマップ => "ブルームとトーンマップ",
            Self::UI => "UI",
            Self::読み戻し => "読み戻し",
        }
    }

    fn 順位(self) -> u8 {
        match self {
            Self::スキニング => 0,
            Self::布シミュレーション => 1,
            Self::影 => 2,
            Self::シーン => 3,
            Self::空 => 4,
            Self::粒子 => 5,
            Self::ブルームとトーンマップ => 6,
            Self::UI => 7,
            Self::読み戻し => 8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct フレーム構成 {
    段階一覧: [Option<フレーム段階>; 段階数上限],
}

impl フレーム構成 {
    pub fn 生成する(段階一覧: &[フレーム段階]) -> Result<Self, フレーム構成エラー> {
        検証する(段階一覧)?;
        let mut 格納先 = [None; 段階数上限];
        for (添字, 段階) in 段階一覧.iter().copied().enumerate() {
            格納先[添字] = Some(段階);
        }
        Ok(Self { 段階一覧: 格納先 })
    }

    pub fn 段階一覧(&self) -> impl Iterator<Item = フレーム段階> + '_ {
        self.段階一覧.iter().flatten().copied()
    }

    pub fn 含む(&self, 対象: フレーム段階) -> bool {
        self.段階一覧().any(|段階| 段階 == 対象)
    }
}

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum フレーム構成エラー {
    #[error("フレーム構成は1段階以上でなければならない")]
    空,
    #[error("フレーム構成は9段階以下でなければならない")]
    段階数超過,
    #[error("フレーム段階が重複しているか依存順序に反している")]
    重複または順序不正,
    #[error("現在のシーン描画には影段階が必要である")]
    影なし,
    #[error("フレーム構成にはシーン段階が必要である")]
    シーンなし,
}
