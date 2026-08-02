//! 律速切り分けの計測で振る軸と、その裁定材料を読むための報告指定を1つの型に集め、引数の読み取りと起動指定への写しを担う。
//! 受け取るのは`ow4-bench`の引数語、返すのは`blitz_app`へ渡す起動指定の語列である。
//! どの軸も指定が無ければ語を1つも足さないため、指定なしの実行は従来条件のまま変わらない。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

#[cfg(test)]
mod condition_tests;

use std::slice::Iter;

#[derive(Default)]
pub(super) struct シャドウ計測指定 {
    /// `--shadow-resolution <テクセル数>`。値の検証は`blitz_app`の値オブジェクトが行う。
    一辺解像度: Option<String>,
    /// `--caster-margin <メートル>`。
    キャスター余白: Option<String>,
    /// `--no-instance-shadow`。群の個体だけを影の候補から外す。地形は影を落とし続ける。
    個体を影から外す: bool,
    /// `--no-shadow-casters`。地形も個体も布も距離区分のパスへ積まない。残るのは深度配列の消去と保存だけである。
    全キャスターを外す: bool,
    /// `--no-instance-lod`。全個体を最詳細段で描く。代表世界の2段はインデックス数が同じで寸法だけが違うため、
    /// この対照が増やすのは投入インデックス数でなくキャスター1体あたりのラスタライズ面積である。
    段選択を止める: bool,
    /// `--camera-yaw <度>`。遠い距離区分の個体密度分布が違う視点を選ぶ。
    カメラ方位度: Option<String>,
    /// `--camera-nudge <メートル>`。方位と組み合わせて視点を経路上の別位置へ移す。
    カメラずれ: Option<String>,
    /// `--max-shadow-distance <メートル>`。多段設定の最大影距離を差し替える(段S2-1の品質方針α)。
    最大影距離: Option<String>,
    /// `--shadow-caster-range <メートル>`。カメラからの中心距離がこの上限を超える個体を影の候補から外す
    /// 計測専用の単純切断である(段S2-1の品質方針β)。
    影の視距離: Option<String>,
    /// `--report-caster-distance`。キャスター候補を距離帯へ振り分けた分布を報告させる。裁定の距離の水準はこの分布から導く。
    キャスター距離分布を報告する: bool,
}

impl シャドウ計測指定 {
    /// 1語を自分の担当として読めたらtrueを返す。読めなければ呼び出し元が次の解釈へ回す。
    pub(super) fn 語を読む(&mut self, 語: &str, 残り: &mut Iter<String>) -> Result<bool, String> {
        match 語 {
            "--shadow-resolution" => self.一辺解像度 = Some(値を読む(語, 残り)?),
            "--caster-margin" => self.キャスター余白 = Some(値を読む(語, 残り)?),
            "--camera-yaw" => self.カメラ方位度 = Some(値を読む(語, 残り)?),
            "--camera-nudge" => self.カメラずれ = Some(値を読む(語, 残り)?),
            "--max-shadow-distance" => self.最大影距離 = Some(値を読む(語, 残り)?),
            "--shadow-caster-range" => self.影の視距離 = Some(値を読む(語, 残り)?),
            "--report-caster-distance" => self.キャスター距離分布を報告する = true,
            "--no-instance-shadow" => self.個体を影から外す = true,
            "--no-shadow-casters" => self.全キャスターを外す = true,
            "--no-instance-lod" => self.段選択を止める = true,
            _ => return Ok(false),
        }
        Ok(true)
    }

    pub(super) fn 起動指定(&self) -> Vec<String> {
        let 値つき = [
            ("--shadow-resolution", self.一辺解像度.as_ref()),
            ("--caster-margin", self.キャスター余白.as_ref()),
            ("--camera-yaw", self.カメラ方位度.as_ref()),
            ("--camera-nudge", self.カメラずれ.as_ref()),
            ("--max-shadow-distance", self.最大影距離.as_ref()),
            ("--shadow-caster-range", self.影の視距離.as_ref()),
        ];
        let mut 語列 = Vec::new();
        for (名前, 値) in 値つき {
            if let Some(値) = 値 {
                語列.push((*名前).to_string());
                語列.push(値.clone());
            }
        }
        let 旗 = [
            ("--no-instance-shadow", self.個体を影から外す),
            ("--no-shadow-casters", self.全キャスターを外す),
            ("--no-instance-lod", self.段選択を止める),
            ("--report-caster-distance", self.キャスター距離分布を報告する),
        ];
        for (名前, 立っているか) in 旗 {
            if 立っているか {
                語列.push((*名前).to_string());
            }
        }
        語列
    }
}

fn 値を読む(引数名: &str, 残り: &mut Iter<String>) -> Result<String, String> {
    残り.next().cloned().ok_or_else(|| format!("{引数名}の次に値が無い"))
}
