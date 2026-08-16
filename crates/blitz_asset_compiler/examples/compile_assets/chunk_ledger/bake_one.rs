//! 対象1件を焼いてファイルへ書き出す局面。返すのは実行時カタログの項目3つと、次の台帳へ記録する
//! ソース依存一式の内容ハッシュである。据え置くと決まった対象はここへ来ない。

use std::path::PathBuf;

use blitz_asset_compiler::{今回の宣言と依存一式の内容ハッシュを求める, 内容ハッシュ, 置いた個体の数};
use blitz_engine::アセットメタデータ;

use super::super::compile_target::コンパイル対象;
use super::one_target::対象1件の仕上げ係;

pub(super) struct 焼き上がり {
    pub(super) 実行時パス: PathBuf,
    pub(super) ソース依存一覧: Vec<PathBuf>,
    pub(super) メタデータ: アセットメタデータ,
    pub(super) 置いた個体の数: 置いた個体の数,
    pub(super) 内容ハッシュ: 内容ハッシュ,
}

impl 対象1件の仕上げ係<'_> {
    pub(super) fn 対象を焼いて書き出す(&self, 対象: &コンパイル対象) -> Result<焼き上がり, String> {
        let 結果 = self.対象をコンパイルする(対象)?;
        let 実行時パス = 対象.生成物を書き出す(&結果.実行時バイト列)?;
        let 宣言 = self
            .コンパイル係
            .安定idが指すソースのパスを参照する(&対象.id)
            .map_err(|誤り| format!("{}の宣言が指すソースを参照できない: {誤り}", 対象.id))?;
        let 内容ハッシュ = 今回の宣言と依存一式の内容ハッシュを求める(宣言, &結果.ソース依存一覧).map_err(|誤り| 誤り.to_string())?;
        Ok(焼き上がり {
            実行時パス,
            ソース依存一覧: 結果.ソース依存一覧,
            メタデータ: 結果.メタデータ,
            置いた個体の数: 結果.置いた個体の数,
            内容ハッシュ,
        })
    }
}
