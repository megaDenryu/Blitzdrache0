//! プロセス境界の綴りと実行の別の対応表。担当するのはこの対応だけであり、別ごとの採取条件は`capture_condition`が持つ。
//!
//! 表を親から分けるのは、検査点が増えるたびにこの一覧だけが伸び、採取条件の組み立てと押し合うためである。

use super::実行の別;
use crate::distant_view::error::遠景構図の検収エラー;

pub(in crate::distant_view) fn 引数を読む(引数一覧: &[String]) -> Result<実行の別, 遠景構図の検収エラー> {
    let [引数] = 引数一覧 else {
        return Err(遠景構図の検収エラー::引数が不正(引数一覧.join(" ")));
    };
    綴りから選ぶ(引数).ok_or_else(|| 遠景構図の検収エラー::引数が不正(引数一覧.join(" ")))
}

fn 綴りから選ぶ(引数: &str) -> Option<実行の別> {
    Some(match 引数 {
        "--capture-reference" => 実行の別::対照を採る,
        "--capture-candidate" => 実行の別::候補を採る,
        "--capture-reference-no-ssao" => 実行の別::Ssaoなし対照を採る,
        "--capture-candidate-no-ssao" => 実行の別::Ssaoなし候補を採る,
        "--capture-candidate-no-distant-shadow" => 実行の別::遠景影なし候補を採る,
        "--capture-reference-no-post" => 実行の別::後処理なし対照を採る,
        "--capture-candidate-no-post" => 実行の別::後処理なし候補を採る,
        "--capture-shadow-reference" => 実行の別::影の対照を採る,
        "--capture-shadow-candidate" => 実行の別::影の候補を採る,
        "--capture-shadow-reference-visibility" => 実行の別::影の対照の可視度を採る,
        "--capture-shadow-candidate-visibility" => 実行の別::影の候補の可視度を採る,
        "--capture-scatter-reference" => 実行の別::散布の対照を採る,
        "--capture-scatter-candidate" => 実行の別::散布の候補を採る,
        "--capture-scatter-reference-no-post" => 実行の別::後処理なし散布の対照を採る,
        "--capture-scatter-candidate-no-post" => 実行の別::後処理なし散布の候補を採る,
        "--bake-scatter-reference" => 実行の別::散布の対照を焼く,
        "--print-plan" => 実行の別::計画を表示する,
        "--judge" => 実行の別::判定する,
        "--judge-shadow" => 実行の別::影を判定する,
        "--judge-scatter" => 実行の別::散布を判定する,
        _ => return None,
    })
}
