//! 生成の指定から、その生成器を起こすcargoのコマンドを組む工程。
//!
//! 指定の定義から分けているのは、定義が「何を作らせるか」を持つのに対し、こちらが「どう起こすか」を持つためである。
//! 生成器を1つ足すと両方へ枝が増えるが、変わる理由は別である。片方は作らせるものの種類、もう片方は起こし方である。

use std::process::Command;

use super::specification::生成の指定;

/// 生成器を収めているクレート。全部の枝が同じクレートのexampleである。
const 生成器のクレート: &str = "blitz_asset_compiler";

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
    pub(super) fn 生成器の名前(&self) -> &'static str {
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
