//! 生成の指定から、その生成器を起こすcargoのコマンドを組む工程。
//!
//! 指定の定義から分けているのは、定義が「何を作らせるか」を持つのに対し、こちらが「どう起こすか」を持つためである。
//! 生成器を1つ足すと両方へ枝が増えるが、変わる理由は別である。片方は作らせるものの種類、もう片方は起こし方である。
//!
//! 組み立てさせる口を走らせる口と別に置くのは、`cargo run`が組み立てと実行を1つにしており、走らせる前に
//! 実行ファイルの中身を読めないためである。生成の新しさを実行ファイルの中身で判定する側がこの口を通る。

use std::path::PathBuf;
use std::process::Command;

use super::error::生成器エラー;
use super::specification::生成の指定;

/// 生成器を収めているクレート。全部の枝が同じクレートのexampleである。
const 生成器のクレート: &str = "blitz_asset_compiler";

/// cargoが例の実行ファイルを置くディレクトリの名前。出力の木の下でこの名前は固定である。
const 例の置き場のディレクトリ名: &str = "examples";

impl 生成の指定<'_> {
    /// この生成器を起こすcargoのコマンド。生成器自身の引数は`完成した生成引数`が続けて並べる。
    pub(super) fn cargoの起動コマンドを組む(&self) -> Command {
        let mut コマンド = Command::new("cargo");
        コマンド.arg("run");
        if self.構築の知らせを伏せるか() {
            コマンド.arg("--quiet");
        }
        コマンド.args(["-p", 生成器のクレート, "--example", self.生成器の名前(), "--"]);
        コマンド
    }

    /// 起こす生成器のexample名。破れの文面もこの名前で生成器を名指す。
    pub(crate) fn 生成器の名前(&self) -> &'static str {
        match self {
            Self::実行時形式を焼く { .. } => "compile_assets",
            Self::ソースアセットを生成する { .. } => "generate_source_assets",
            Self::入力するglTFの契約を検査する { .. } => "check_glb",
            Self::部品カタログを組み上げる { .. } => "part_catalog",
            Self::組み立てを正解表と突き合わせる { .. } => "part_assembly",
            Self::アセットの変更を見張る { .. } => "watch_assets",
        }
    }

    /// cargo自身の構築の知らせを伏せるか。検査の結果だけを読む入口が伏せる。
    fn 構築の知らせを伏せるか(&self) -> bool {
        matches!(
            self,
            Self::入力するglTFの契約を検査する { .. } | Self::部品カタログを組み上げる { .. } | Self::組み立てを正解表と突き合わせる { .. }
        )
    }
}

impl 生成の指定<'_> {
    /// 生成器を組み立てさせ、組み上がった実行ファイルのパスを返す。走らせはしない。
    ///
    /// 置き場をcargoの出力の綴りから読まず、自分の実行ファイルの隣の`examples`から導くのは、
    /// xtask自身とこの生成器が同じ出力の木の同じ形の下に置かれるためである。
    pub fn 生成器を組み立てて実行ファイルのパスを求める(&self) -> Result<PathBuf, 生成器エラー> {
        let 生成器 = self.生成器の名前();
        let 状態 = Command::new("cargo")
            .args(["build", "--quiet", "-p", 生成器のクレート, "--example", 生成器])
            .status()
            .map_err(|誤り| 生成器エラー::生成器を起こせなかった { 生成器, 誤り })?;
        if !状態.success() {
            return Err(生成器エラー::生成器が失敗して終わった {
                生成器,
                終了状態: 状態.to_string(),
            });
        }
        Ok(self
            .実行ファイルの置き場を求める()?
            .join(format!("{生成器}{}", std::env::consts::EXE_SUFFIX)))
    }

    fn 実行ファイルの置き場を求める(&self) -> Result<PathBuf, 生成器エラー> {
        let 自分の実行ファイル =
            std::env::current_exe().map_err(|誤り| 生成器エラー::生成器の実行ファイルの置き場を決められない {
                生成器: self.生成器の名前(),
                誤り,
            })?;
        let 置き場 = 自分の実行ファイル.parent().map(|親| 親.join(例の置き場のディレクトリ名));
        置き場.ok_or_else(|| 生成器エラー::生成器の実行ファイルの置き場を決められない {
            生成器: self.生成器の名前(),
            誤り: std::io::Error::other("自分の実行ファイルが親のディレクトリを持たない"),
        })
    }
}
