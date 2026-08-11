//! 順3-Ic-3aの検収入口。遠方環境の契約を選ぶ世界を1つのプロセスで走らせ、複製した検収用シェーダーディレクトリの
//! 定数近似側だけを書き換えて、契約別のシェーダー束の完全交換が絵を1画素も動かさないことを確かめる。
//!
//! 同時に3つを要求する。差し替えが成功した行がちょうど1回出ること、validationの指摘が0件であること、
//! 差し替えの前後の読み戻し画像がバイト一致することである。画像一致だけを見ると、監視も再コンパイルも交換も
//! 1度も起きなかった実行が「絵が変わらなかった」だけで通ってしまう。成功の行を併置してその偽陽性を塞ぐ。
//!
//! 書き換えるのが定数近似側だけなのは、この世界のパイプラインが遠方環境の画素段を選ぶためである。旧来の
//! 単一のシェーダー一式による差し替えは契約を見ずに全キーを組み直したため、この書き換えで間接光が黒へ落ちた。
//! 完全交換では書き換えた側が1本も載らず、絵が動かないことがその根拠になる。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Ic-3の実装段割り」

mod judgment;
mod run;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/shader_reload_draw";
/// 検収用に複製するシェーダーディレクトリ。リポジトリ本体の`shaders/`を監視先にすると、この検収の書き換えが
/// リポジトリのファイルを変えてしまう。
const シェーダーコピー先: &str = "target/shader_reload_shaders";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] shader-reload-draw成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] shader-reload-draw失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let 監視先 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 実行 = run::差し替えを挟んで描画する(&出力先, &監視先)?;
    judgment::差し替えを検査する(&実行)
}
