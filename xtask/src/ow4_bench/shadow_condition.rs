//! 律速切り分けの計測で振る軸を1つの型に集め、引数の読み取りと起動指定への写しを担う。
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
    /// `--no-instance-shadow`。キャスター0の対照であり、影を落とすのが地形だけになる。
    キャスターを外す: bool,
    /// `--no-instance-lod`。全個体を最詳細段で描き、投入頂点量だけを増やす対照になる。
    段選択を止める: bool,
    /// `--camera-yaw <度>`。遠い距離区分の個体密度分布が違う視点を選ぶ。
    カメラ方位度: Option<String>,
    /// `--camera-nudge <メートル>`。方位と組み合わせて視点を経路上の別位置へ移す。
    カメラずれ: Option<String>,
}

impl シャドウ計測指定 {
    /// 1語を自分の担当として読めたらtrueを返す。読めなければ呼び出し元が次の解釈へ回す。
    pub(super) fn 語を読む(&mut self, 語: &str, 残り: &mut Iter<String>) -> Result<bool, String> {
        match 語 {
            "--shadow-resolution" => self.一辺解像度 = Some(値を読む(語, 残り)?),
            "--caster-margin" => self.キャスター余白 = Some(値を読む(語, 残り)?),
            "--camera-yaw" => self.カメラ方位度 = Some(値を読む(語, 残り)?),
            "--camera-nudge" => self.カメラずれ = Some(値を読む(語, 残り)?),
            "--no-instance-shadow" => self.キャスターを外す = true,
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
        ];
        let mut 語列 = Vec::new();
        for (名前, 値) in 値つき {
            if let Some(値) = 値 {
                語列.push((*名前).to_string());
                語列.push(値.clone());
            }
        }
        let 旗 = [
            ("--no-instance-shadow", self.キャスターを外す),
            ("--no-instance-lod", self.段選択を止める),
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
