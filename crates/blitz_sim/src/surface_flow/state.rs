//! 表面流格子の不変状態。時間発展は新しい状態を返す。

use super::cell::表面セル;
use super::error::表面流仕様エラー;
use super::spec::表面流仕様;

#[derive(Debug, Clone, PartialEq)]
pub struct 表面流状態 {
    格子寸法: [u32; 2],
    セル一覧: Vec<表面セル>,
}

impl 表面流状態 {
    pub fn 空で生成する(仕様: &表面流仕様) -> Self {
        let [列数, 行数] = 仕様.格子寸法();
        let 件数 = usizeへ(列数) * usizeへ(行数);
        Self {
            格子寸法: [列数, 行数],
            セル一覧: vec![表面セル::default(); 件数],
        }
    }

    pub fn 液膜を設定する(&self, 座標: [u32; 2], 液膜厚さ: f32) -> Result<Self, 表面流仕様エラー> {
        if !液膜厚さ.is_finite() || 液膜厚さ < 0.0 {
            return Err(表面流仕様エラー::液膜厚さが不正 { 指定値: 液膜厚さ });
        }
        let mut 次 = self.clone();
        let 添字 = self.添字を得る(座標)?;
        let 速度 = 次.セル一覧[添字].接線速度();
        次.セル一覧[添字] = 表面セル::生成する(液膜厚さ, 速度);
        Ok(次)
    }

    pub fn 液膜分布で生成する(
        仕様: &表面流仕様, 液膜厚さを得る: impl Fn([u32; 2]) -> f32
    ) -> Result<Self, 表面流仕様エラー> {
        let [列数, 行数] = 仕様.格子寸法();
        let mut セル一覧 = Vec::with_capacity(usizeへ(列数) * usizeへ(行数));
        for 行 in 0..行数 {
            for 列 in 0..列数 {
                let 液膜厚さ = 液膜厚さを得る([列, 行]);
                if !液膜厚さ.is_finite() || 液膜厚さ < 0.0 {
                    return Err(表面流仕様エラー::液膜厚さが不正 { 指定値: 液膜厚さ });
                }
                セル一覧.push(表面セル::生成する(液膜厚さ, [0.0, 0.0]));
            }
        }
        Ok(Self::構築する([列数, 行数], セル一覧))
    }

    pub fn セル(&self, 座標: [u32; 2]) -> Result<表面セル, 表面流仕様エラー> {
        self.添字を得る(座標).map(|添字| self.セル一覧[添字])
    }

    pub fn 総液膜厚さ(&self) -> f32 {
        self.セル一覧.iter().map(表面セル::液膜厚さ).sum()
    }

    pub(crate) fn セル一覧(&self) -> &[表面セル] {
        &self.セル一覧
    }
    pub(crate) fn 格子寸法(&self) -> [u32; 2] {
        self.格子寸法
    }
    pub(crate) fn 構築する(格子寸法: [u32; 2], セル一覧: Vec<表面セル>) -> Self {
        Self { 格子寸法, セル一覧 }
    }

    fn 添字を得る(&self, [列, 行]: [u32; 2]) -> Result<usize, 表面流仕様エラー> {
        let [列数, 行数] = self.格子寸法;
        if 列 >= 列数 || 行 >= 行数 {
            return Err(表面流仕様エラー::セル座標が範囲外 { 列, 行 });
        }
        Ok(usizeへ(行) * usizeへ(列数) + usizeへ(列))
    }
}

pub(crate) fn usizeへ(値: u32) -> usize {
    match usize::try_from(値) {
        Ok(結果) => 結果,
        Err(_) => panic!("u32がusizeへ変換できないプラットフォームは対象外: {値}"),
    }
}
