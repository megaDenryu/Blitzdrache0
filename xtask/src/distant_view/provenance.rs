//! 採取1回の由来ファイル。担当するのは「契約の行・構築の由来・採取の旗・構図の実測を1つのファイルへ書くこと」である。
//! 受け取るのは採取条件と構築の由来と構図の実測、返すのは書き込みの成否である。
//!
//! 旗の行そのものの綴りと読み戻しは`capture_flags`が持つ。ここはファイルへ何をどの順で並べるかだけを決める。

mod capture_flags;

pub(in crate::distant_view) use capture_flags::採取の旗;

use std::path::{Path, PathBuf};

use super::error::遠景構図の検収エラー;
use super::plan::採取条件;

pub(super) fn 由来を書く(
    置き場: &Path,
    条件: &採取条件,
    由来: &crate::release_build::構築の由来,
    構図の実測: &str,
) -> Result<(), 遠景構図の検収エラー> {
    let パス = 置き場.join(format!("{}.txt", 条件.名前));
    let mut 行一覧 = vec![format!("distant-view-contract=v1 {}", super::plan::計画を表示する())];
    行一覧.push(採取の旗::採取条件から作る(条件).行にする());
    行一覧.extend(由来.注記一覧());
    行一覧.push(format!("view-measurement={構図の実測}"));
    std::fs::write(&パス, 行一覧.join("\n")).map_err(|誤り| 遠景構図の検収エラー::由来を書けなかった {
        パス: PathBuf::from(&パス),
        誤り,
    })
}
