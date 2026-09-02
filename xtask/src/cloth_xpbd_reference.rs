//! XPBD共通拘束基盤の実装順3(Issue #36)の検収入口。布の構造とせん断の距離拘束を本来のXPBD(グラフ彩色)で解くGPUの経路を、
//! 同じ初期条件・同じ刻み数・同じ反復回数のCPUの参照計算(正典式)と突き合わせ、あわせて布の1刻みのGPU時間を採る。
//!
//! 布は自己衝突に触れない32x32の格子であり、床にも触れない高さから吊るした題材(アプリの`--cloth-xpbd-reference`)をコンプライアンス0(硬い)と
//! 有限の値の2条件で、同じ布の下端の1粒子を床の下(y=−0.01メートル)の目標へ固定した題材(`--cloth-xpbd-reference-below-floor`)を
//! コンプライアンス0の1条件で走らせる。3つ目の条件は、床の後の目標拘束の最終の成立が無いとその粒子が床の高さに残ってCPUと0.01メートル食い違う回帰を検出する
//! (Issue #37のcodexレビューの是正)。判定は5つである。validationの指摘が0件であること、読み戻した位置と乗数がすべて有限であること、
//! CPUの参照計算との位置と乗数の最大差が許容差に収まること、参照計算が自己衝突の条件へ入っていないこと(差の行が在ること)、
//! 布の1刻みのGPU時間の合計がシミュレーションの枠(`_doc/計画/評価軸.md` 3.1の3.0ミリ秒)に収まることである。
//! 描く世界は`cargo xtask cloth-empty`と同じ「群が両方の視錐台の外にある世界」である。判定の中身は`judgment`、行の読み取りは`parse`にある。

mod error;
mod gpu_time;
mod judgment;
mod parse;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use error::布のXPBD参照比較の検収エラー;

use crate::acceptance::{描画フレーム数, 描画検収の実行環境, 検収の1回の実行, 検収の実行名, 検収シーン名};
use crate::vegetation_run;

const 出力ディレクトリ: &str = "target/cloth_xpbd_reference";
const シェーダーコピー先: &str = "target/cloth_xpbd_reference_shaders";
const シーン: 検収シーン名 = 検収シーン名::生成する("instance_all_culled");
/// 突き合わせる刻み数。長い実行では単精度の演算順の差が力学で増幅されるため短くする(`xpbd-solver-bench`の比較の刻み数10と同じ桁)。
const 比較のフレーム数: 描画フレーム数 = 描画フレーム数::生成する(12);
/// GPU時間の窓(60フレーム)が満ちる枚数。
const 計測のフレーム数: 描画フレーム数 = 描画フレーム数::生成する(120);
const 共通の引数: [&str; 1] = ["--no-post"];
/// 有限のコンプライアンス(メートル毎ニュートン)。刻み依存量α̃=0.01×3600=36毎キログラムであり、有効逆質量512の約7%として乗数の項が式へ効く。
const 有限のコンプライアンス: &str = "0.01";
/// 参照比較の3条件(実行名・アプリの引数名・コンプライアンス)。
const 参照比較の条件一覧: [(&str, &str, &str); 3] = [
    ("hard", "--cloth-xpbd-reference", "0"),
    ("soft", "--cloth-xpbd-reference", 有限のコンプライアンス),
    ("floor", "--cloth-xpbd-reference-below-floor", "0"),
];

pub fn 布のxpbd参照比較を確認する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] cloth-xpbd-reference成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] cloth-xpbd-reference失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, 布のXPBD参照比較の検収エラー> {
    if !crate::gen_source_assets::検証用ソースアセットを生成して成否を返す() || !crate::compile_assets::植生世界を既定で生成する()
    {
        return Err(布のXPBD参照比較の検収エラー::検証用アセットを生成できなかった);
    }
    let 実行環境 = vegetation_run::植生世界の実行環境を作る(PathBuf::from(出力ディレクトリ))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))
        .map_err(布のXPBD参照比較の検収エラー::シェーダーの一時コピーを作れなかった)?;

    let mut 要約 = Vec::new();
    for (名前, 引数名, コンプライアンス) in 参照比較の条件一覧 {
        let 実行 = 報告を採る(&実行環境, 名前, &シェーダー入口, 比較のフレーム数, &[引数名, コンプライアンス])?;
        let 観測 = parse::参照比較を読む(実行.報告())?;
        judgment::参照比較を判定する(名前, &観測)?;
        要約.push(観測.要約(名前));
    }
    let 実行 = 報告を採る(
        &実行環境,
        "gpu_time",
        &シェーダー入口,
        計測のフレーム数,
        &["--cloth", "--report-gpu-times"],
    )?;
    let 一刻み = gpu_time::布の一刻みを読む(実行.報告())?;
    judgment::一刻みのgpu時間を判定する(&一刻み)?;
    要約.push(一刻み.要約());
    Ok(要約.join("。"))
}

fn 報告を採る(
    実行環境: &描画検収の実行環境,
    名前: &str,
    シェーダー入口: &Path,
    フレーム数: 描画フレーム数,
    追加引数: &[&str],
) -> Result<検収の1回の実行, 布のXPBD参照比較の検収エラー> {
    let mut 引数 = 共通の引数.to_vec();
    引数.extend_from_slice(追加引数);
    let 実行 = 実行環境.描いて読み戻す(
        検収の実行名::生成する(名前)?,
        &vegetation_run::植生世界の起動指定を組み立てる(シーン, フレーム数, シェーダー入口, &引数),
    )?;
    実行.報告().画面へ流す();
    Ok(実行)
}
