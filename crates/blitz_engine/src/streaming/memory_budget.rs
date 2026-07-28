//! 現在使用量を含むRAM・VRAM上限へ中心優先の接頭辞を収容する。

use super::{
    memory_amount::ストリーミングメモリ量,
    memory_candidate::チャンク予算候補,
    memory_result::{ストリーミング予算結果, 予算判定},
};

#[derive(Debug, Clone, Copy)]
pub struct ストリーミング予算 {
    上限: ストリーミングメモリ量,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ストリーミング予算エラー {
    #[error("RAMとVRAMの予算は1バイト以上でなければならない")]
    上限ゼロ,
    #[error("現在のRAMまたはVRAM使用量が予算を超えている")]
    現在使用量超過,
}

impl ストリーミング予算 {
    pub fn 生成する(上限: ストリーミングメモリ量) -> Result<Self, ストリーミング予算エラー> {
        if 上限.ramバイト数 == 0 || 上限.vramバイト数 == 0 {
            return Err(ストリーミング予算エラー::上限ゼロ);
        }
        Ok(Self { 上限 })
    }

    pub fn 上限(self) -> ストリーミングメモリ量 {
        self.上限
    }

    /// 現在使用量が上限を超えているか。超えている状態で`適用する`を呼ぶと入力状態のエラーになるため、呼出し側はこれで先に分岐する。
    pub fn 上限を超えるか(self, 使用量: ストリーミングメモリ量) -> bool {
        使用量.いずれかが上限を超えるか(self.上限)
    }

    pub fn 適用する(
        self,
        現在使用量: ストリーミングメモリ量,
        候補一覧: &[チャンク予算候補],
    ) -> Result<ストリーミング予算結果, ストリーミング予算エラー> {
        if self.上限を超えるか(現在使用量) {
            return Err(ストリーミング予算エラー::現在使用量超過);
        }
        let mut 使用量 = 現在使用量;
        let mut 収容一覧 = Vec::new();
        let mut 除外開始 = 候補一覧.len();
        for (添字, 候補) in 候補一覧.iter().enumerate() {
            let Some(追加後) = 使用量.加算する(候補.メモリ量()) else {
                除外開始 = 添字;
                break;
            };
            if 追加後.いずれかが上限を超えるか(self.上限) {
                除外開始 = 添字;
                break;
            }
            使用量 = 追加後;
            収容一覧.push(*候補);
        }
        Ok(結果を作る(候補一覧, 除外開始, 使用量, self.上限, 収容一覧))
    }
}

fn 結果を作る(
    候補一覧: &[チャンク予算候補],
    除外開始: usize,
    使用量: ストリーミングメモリ量,
    上限: ストリーミングメモリ量,
    収容一覧: Vec<チャンク予算候補>,
) -> ストリーミング予算結果 {
    let 除外一覧: Vec<_> = 候補一覧[除外開始..].iter().map(|候補| 候補.要求().座標()).collect();
    let 判定 = if let Some(最初) = 候補一覧.get(除外開始) {
        let 追加 = 最初.メモリ量();
        予算判定::縮退 {
            最初の除外: 最初.要求().座標(),
            ram超過: 使用量.ramバイト数.checked_add(追加.ramバイト数).is_none_or(|値| 値 > 上限.ramバイト数),
            vram超過: 使用量.vramバイト数.checked_add(追加.vramバイト数).is_none_or(|値| 値 > 上限.vramバイト数),
        }
    } else {
        予算判定::全件収容
    };
    ストリーミング予算結果 {
        収容一覧,
        除外一覧,
        使用量,
        判定,
    }
}
