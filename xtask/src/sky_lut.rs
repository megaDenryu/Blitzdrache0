//! 大気LUT生成パス数と計器の完了ゲートの入口。大気LUT腕で地形世界を描き、大気LUT生成パスの実行数が
//! 更新判定どおりであることと、GPU計器区間が別々に立つこと、毎フレーム走る区間の合計が空の定常予算に収まること、
//! validationが0件であることを確かめる。
//!
//! 数えるのは指示ではなくレンダーグラフへ積まれたパスの本数である(`blitz_render`の大気LUT生成パス数の記録)。
//! 時計を止めた条件と進めた条件の2つを走らせるのは、片方だけでは「常に0本」「常に4本」という誤りが検出できないためである。
//! 止めた条件は初回の4本の後が全部0本、進めた条件は初回の4本の後がスカイビューと空中遠近の2本であることを要求する。
//! 本数の並びを見る前に列そのものの整合(検収の160フレームを数えたか・列の長さと総数が互いに合うか)を確かめるのは、
//! 並びだけの判定が短い実行の列[4]や[4, 2]も通してしまうためである。
//! 3つめの条件として合成を切った実行を走らせ、起動指定1つで空中遠近を入れる前の描画へ戻れる停止点が
//! 成立していることを確かめる(判定の中身と、保存した基準画像を使わない理由は`stop_point`にある)。
//! 一日内時刻を引数で選べるのは、絵の目視が代表時刻ごとに要るためである(判定は時刻に依らず同じ)。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「更新判定の置き場所」「予算と計器」

mod gpu_time;
mod judgment;
mod pass_count;
mod run;
mod series_integrity;
mod stop_point;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/sky_lut";

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    let 時刻 = match 一日内秒を読む(引数一覧) {
        Ok(時刻) => 時刻,
        Err(理由) => {
            eprintln!("[xtask] sky-lut失敗: {理由}");
            return ExitCode::FAILURE;
        }
    };
    match 検収する(時刻.as_deref()) {
        Ok(要約) => {
            println!("[xtask] sky-lut成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] sky-lut失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

/// `--time-of-day <秒>`があればその値を返す。指定が無ければ世界の方針の既定時刻(11時)で走る。
fn 一日内秒を読む(引数一覧: &[String]) -> Result<Option<String>, String> {
    let Some(位置) = 引数一覧.iter().position(|引数| 引数 == "--time-of-day") else {
        return Ok(None);
    };
    let 値 = 引数一覧.get(位置 + 1).ok_or_else(|| "--time-of-dayの後に秒が無い".to_string())?;
    値.parse::<f64>()
        .map_err(|誤り| format!("--time-of-dayの値を数として読めない({値}): {誤り}"))?;
    Ok(Some(値.clone()))
}

fn 検収する(一日内秒: Option<&str>) -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::地形世界を既定で生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;

    let 停止 = run::描画する(&出力先, "lut_stopped", run::条件::時計停止, 一日内秒)?;
    let 進行 = run::描画する(&出力先, "lut_advancing", run::条件::時計進行, 一日内秒)?;
    let 合成なし = run::描画する(&出力先, "lut_no_composite", run::条件::合成なし, 一日内秒)?;
    let 停止の列 = pass_count::読む(&停止.標準出力, "時計停止")?;
    let 進行の列 = pass_count::読む(&進行.標準出力, "時計進行")?;
    let 合成なしの列 = pass_count::読む(&合成なし.標準出力, "合成なし")?;

    series_integrity::列の整合を確かめる(&停止の列, "時計停止")?;
    series_integrity::列の整合を確かめる(&進行の列, "時計進行")?;
    series_integrity::列の整合を確かめる(&合成なしの列, "合成なし")?;
    judgment::時計停止を判定する(&停止の列)?;
    judgment::時計進行を判定する(&進行の列)?;
    let 停止点 = stop_point::停止点を判定する(&合成なし.標準出力, &合成なしの列, &合成なし.画素バイト列, &停止.画素バイト列)?;
    let 計器 = gpu_time::区間を読む(&停止.標準出力)?;

    let png = crate::raw_png::変換する(&出力先.join("lut_stopped"))?;
    let 合成なしのpng = crate::raw_png::変換する(&出力先.join("lut_no_composite"))?;
    Ok(format!(
        "一日内時刻{}で、時計停止は{}フレームで生成パス{}本(列の先頭{:?})、時計進行は{}フレームで生成パス{}本(うちスカイビューと空中遠近の2本を焼いたフレームが{}回)、計器は{}、{}、停止点は{}、絵は{}と{}",
        一日内秒.map_or("既定(11時)".to_string(), |秒| format!("{秒}秒")),
        停止の列.数えたフレーム数,
        停止の列.総数,
        停止の列.先頭の並び(4),
        進行の列.数えたフレーム数,
        進行の列.総数,
        進行の列.本数の回数(2),
        計器.区間の並び,
        計器.予算との比較(),
        停止点,
        png.display(),
        合成なしのpng.display()
    ))
}
