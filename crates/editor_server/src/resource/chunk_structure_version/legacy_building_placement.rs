//! 旧版の建物配置1件を最新の形へ写す工程。受け取るのは種別で建物を区別していた旧版の1件であり、
//! 返すのは建物定義IDを持つ現在の`建物の配置`である。対応する建物定義が無い種別は黙って別の建物へ
//! 変えず、移行エラーにする(参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断10」)。

use serde::Deserialize;

use super::チャンク構造移行エラー;
use crate::resource::building::建物の配置;
use crate::resource::building_definition_id::建物定義ID;
use crate::resource::position::位置3次元;

/// 旧版の家屋を移す先の建物定義ID。カタログの正本に実在することは`移行先の建物定義IDは正本に実在する`が見る。
const 家屋の移行先ID: &str = "frame_house_one_bay";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub(super) enum 旧版の建物種別 {
    家屋,
    塔,
    宝箱,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(super) struct 種別を持つ旧版の建物配置 {
    識別子: String,
    種別: 旧版の建物種別,
    位置: 位置3次元,
    向きラジアン: f64,
    基礎半径メートル: f64,
    なじみ半径メートル: f64,
}

impl 種別を持つ旧版の建物配置 {
    fn 最新の形へ変換する(self) -> Result<建物の配置, チャンク構造移行エラー> {
        let 対応する建物定義が無い = || チャンク構造移行エラー::対応する建物定義が無い {
            建物識別子: self.識別子.clone(),
            旧種別: format!("{:?}", self.種別),
        };
        let 移行先の建物定義 = match self.種別 {
            旧版の建物種別::家屋 => 建物定義ID::生成する(家屋の移行先ID).map_err(|_| 対応する建物定義が無い())?,
            旧版の建物種別::塔 | 旧版の建物種別::宝箱 => return Err(対応する建物定義が無い()),
        };
        Ok(建物の配置 {
            識別子: self.識別子,
            建物定義ID: 移行先の建物定義,
            位置: self.位置,
            向きラジアン: self.向きラジアン,
            基礎半径メートル: self.基礎半径メートル,
            なじみ半径メートル: self.なじみ半径メートル,
        })
    }
}

pub(super) fn 旧版の建物一覧を最新の形へ変換する(
    旧一覧: Vec<種別を持つ旧版の建物配置>,
) -> Result<Vec<建物の配置>, チャンク構造移行エラー> {
    旧一覧.into_iter().map(種別を持つ旧版の建物配置::最新の形へ変換する).collect()
}

#[cfg(test)]
mod tests {
    use super::家屋の移行先ID;

    /// 移行先の綴りは別クレートにある建物定義の正本と独立に書かれているため、機械で突き合わせる。
    /// 正本の識別子一覧は外部アセットの置き場に依らず答えられるので、この照合はどの環境でも走る。
    #[test]
    fn 移行先の建物定義の識別子は正本に実在する() {
        let 識別子一覧 = blitz_asset_compiler::建物定義の識別子一覧();
        assert!(
            識別子一覧.contains(&家屋の移行先ID),
            "旧版の家屋の移行先{家屋の移行先ID}が建物定義の正本に無い(正本: {識別子一覧:?})"
        );
    }
}
