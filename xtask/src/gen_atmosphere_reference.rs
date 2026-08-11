//! Bruneton 2017実装のCPU参照から、大気の物理量の期待値を焼く工程。
//! 受け取るのは`precomputed_atmospheric_scattering`の作業コピーのパス、書き出すのは
//! `crates/blitz_render/data/bruneton_reference.txt`である。
//!
//! 出典: Eric Bruneton, "Precomputed Atmospheric Scattering: a New Implementation" (2017)の公開実装
//! (https://github.com/ebruneton/precomputed_atmospheric_scattering、3条項BSDライセンス)。
//! 焼き出しの対象として固定したリビジョンは`revision.rs`が定数で持ち、作業コピーの実測値と一致しなければ生成を拒む。
//! 作業コピーの追跡下のファイルに未コミットの変更があるときも、焼いたコードと出自の表示が食い違うため同様に拒む。
//! 参照実装のコードそのものはこのリポジトリへ取り込まず、その出力である数値だけを焼く。
//!
//! 手順(いずれもリポジトリルートで実行する):
//!   git clone https://github.com/ebruneton/precomputed_atmospheric_scattering <作業パス>
//!   git -C <作業パス> submodule update --init external/dimensional_types
//!   cargo xtask gen-atmosphere-reference <作業パス>
//!
//! 参照実装はGLSLをC++として取り込む構成であり、構築にはMSVCのC++ツールセットが要る。
//! 構築手順は`xtask/reference/build_bruneton_dump.bat`が持ち、そこからvswhere経由でvcvarsを呼ぶ。
//! この経路はWindows専用である。参照が焼けない環境では、大気数学の検証は解析的な閉形式との一致で行う。

mod build;
mod emit;
mod error;
mod revision;

use std::path::Path;
use std::process::ExitCode;

use error::大気の期待値の焼き出しエラー;

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    let Some(参照パス) = 引数一覧.first() else {
        eprintln!("[xtask] gen-atmosphere-reference失敗: precomputed_atmospheric_scatteringの作業コピーのパスを引数に渡すこと");
        return ExitCode::FAILURE;
    };
    match 焼く(Path::new(参照パス)) {
        Ok(要約) => {
            println!("[xtask] gen-atmosphere-reference成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] gen-atmosphere-reference失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 焼く(参照パス: &Path) -> Result<String, 大気の期待値の焼き出しエラー> {
    let リビジョン = revision::焼き出しの出自を確かめる(参照パス)?;
    let 実行ファイル = build::焼き出しを構築する(参照パス)?;
    let 本文 = build::焼き出しを実行する(&実行ファイル)?;
    emit::書き出す(&本文, &リビジョン)
}
