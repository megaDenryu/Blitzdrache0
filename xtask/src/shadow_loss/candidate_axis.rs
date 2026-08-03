//! 候補として振る1つの軸。担当するのは、軸の綴りの読み取りと、値を添えた候補の組み立てと、
//! `blitz_app`へ渡す起動指定への変換である。
//!
//! αとβを1つの型の2つの枝で表すのは、設計の正本が「αとβを同時に変えない」と定めるためである。
//! 2つを別々の任意項目で受けると、両方を渡した実行も1つも渡さない実行も型の上では正しくなり、
//! どちらの軸が絵を動かしたか分けられない数をそのまま裁定材料にしてしまう。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

use super::distance::距離メートル;

const 最大影距離の綴り: &str = "--max-shadow-distance";
const 影の視距離の綴り: &str = "--shadow-caster-range";

/// 候補で振る軸そのもの。距離を持たない側であり、綴りの読み取りと、構図が受け入れるかの判定に使う。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum 計測軸 {
    /// α。多段設定の最大影距離を差し替える。
    最大影距離,
    /// β。カメラからの中心距離で影の候補を切る。
    影の視距離,
}

impl 計測軸 {
    pub(super) const 全軸: [Self; 2] = [Self::最大影距離, Self::影の視距離];

    pub(super) fn 綴り(self) -> &'static str {
        match self {
            Self::最大影距離 => 最大影距離の綴り,
            Self::影の視距離 => 影の視距離の綴り,
        }
    }

    pub(super) fn 綴りから読む(語: &str) -> Option<Self> {
        Self::全軸.into_iter().find(|軸| 軸.綴り() == 語)
    }

    pub(super) fn 綴りを並べる(軸一覧: &[Self]) -> String {
        軸一覧
            .iter()
            .map(|軸| format!("{} メートル", 軸.綴り()))
            .collect::<Vec<String>>()
            .join(" または ")
    }

    pub(super) fn 値を添える(self, 綴り: &str) -> Result<候補の計測指定, String> {
        let 距離 = 距離メートル::生成する(綴り)?;
        Ok(match self {
            Self::最大影距離 => 候補の計測指定::最大影距離(距離),
            Self::影の視距離 => 候補の計測指定::影の視距離(距離),
        })
    }
}

/// 候補として振る軸と、その距離。ちょうど1つの軸だけを持つ。
pub(super) enum 候補の計測指定 {
    最大影距離(距離メートル),
    影の視距離(距離メートル),
}

impl 候補の計測指定 {
    pub(super) fn 軸(&self) -> 計測軸 {
        match self {
            Self::最大影距離(_) => 計測軸::最大影距離,
            Self::影の視距離(_) => 計測軸::影の視距離,
        }
    }

    pub(super) fn 綴り(&self) -> &'static str {
        self.軸().綴り()
    }

    pub(super) fn 起動指定へ写す(&self) -> Vec<String> {
        let (Self::最大影距離(距離) | Self::影の視距離(距離)) = self;
        vec![self.綴り().to_string(), 距離.綴り().to_string()]
    }
}
