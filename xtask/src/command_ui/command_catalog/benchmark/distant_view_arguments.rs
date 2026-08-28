//! 遠景の撮影と判定が受け取る、ちょうど1つの実行の別の綴りの一覧。1コマンドで独立したファイルへ置くのは、
//! この1件だけで22の綴りを持ち、分類の一覧と同居させると一覧の見通しを潰すためである。
//! 綴りと実行の別の対応の正本は`xtask/src/distant_view/plan/argument_name.rs`が持つ。

use super::super::argument::省略したときの扱い::受け付けずに聞き直す;
use super::super::argument::{引数定義, 選択肢};

const 実行の別の選択肢一覧: &[選択肢] = &[
    選択肢::生成する("--capture-reference", "対照の絵を採る"),
    選択肢::生成する("--capture-candidate", "候補の絵を採る"),
    選択肢::生成する("--capture-reference-no-ssao", "画面空間の環境遮蔽を切った対照の絵を採る"),
    選択肢::生成する("--capture-candidate-no-ssao", "画面空間の環境遮蔽を切った候補の絵を採る"),
    選択肢::生成する("--capture-candidate-no-distant-shadow", "遠景の影を切った候補の絵を採る"),
    選択肢::生成する("--capture-reference-no-post", "ポスト処理を切った対照の絵を採る"),
    選択肢::生成する("--capture-candidate-no-post", "ポスト処理を切った候補の絵を採る"),
    選択肢::生成する(
        "--capture-shadow-reference",
        "影の検査点のための対照の絵を、ポスト処理と明示境界を切って採る",
    ),
    選択肢::生成する("--capture-shadow-candidate", "影の検査点のための候補の絵を、ポスト処理を切って採る"),
    選択肢::生成する("--capture-shadow-reference-visibility", "影の対照の影可視度を可視化して採る"),
    選択肢::生成する("--capture-shadow-candidate-visibility", "影の候補の影可視度を可視化して採る"),
    選択肢::生成する("--capture-scatter-reference", "散布を焼かない対照のアセットから対照の絵を採る"),
    選択肢::生成する("--capture-scatter-candidate", "散布の候補の絵を採る"),
    選択肢::生成する("--capture-scatter-reference-no-post", "ポスト処理を切って散布の対照の絵を採る"),
    選択肢::生成する(
        "--capture-scatter-reference-bare",
        "ポスト処理と画面空間の環境遮蔽と影のキャスターを切って散布の対照の絵を採る",
    ),
    選択肢::生成する(
        "--capture-scatter-candidate-bare",
        "ポスト処理と画面空間の環境遮蔽と影のキャスターを切って散布の候補の絵を採る",
    ),
    選択肢::生成する("--capture-scatter-candidate-no-post", "ポスト処理を切って散布の候補の絵を採る"),
    選択肢::生成する("--bake-scatter-reference", "散布を焼かない対照のアセットを焼く(絵は採らない)"),
    選択肢::生成する("--print-plan", "撮影の計画だけを表示する"),
    選択肢::生成する("--judge", "遠景の検査点を2段で判定する"),
    選択肢::生成する("--judge-shadow", "影の距離区分の再配分の検査点を、深度一致と影可視度への帰属で判定する"),
    選択肢::生成する("--judge-scatter", "散布の検査点を機構ごとの3段で判定する"),
];

pub(super) const 遠景の撮影と判定の引数定義: &[引数定義] = &[引数定義::綴りから1つ選ぶ値を定義する(
    None,
    "実行の別",
    "絵を採るか、対照のアセットを焼くか、判定するか、計画を表示するかを1つ選ぶ。ちょうど1つだけ渡せる",
    受け付けずに聞き直す,
    実行の別の選択肢一覧,
)];
