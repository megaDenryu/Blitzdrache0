//! OW3の統合DoD入口。既存の原点移動検査とGPU継ぎ目検査に続き、半径2の本番ストリーミング経路で容量・解除・LOD変更を測る。

mod metrics;
mod run;

use std::process::{Command, ExitCode};

const 上限バイト数: u64 = 16 * 1024 * 1024;

pub fn 実行する() -> ExitCode {
    if !子xtaskを実行する("origin-invariance") || !子xtaskを実行する("lod-crack") {
        return ExitCode::FAILURE;
    }
    let Some(実行結果) = run::統合経路を実行する() else {
        return ExitCode::FAILURE;
    };
    print!("{}", 実行結果.標準出力);
    eprint!("{}", 実行結果.標準エラー);
    let 計測 = match metrics::読み取る(&実行結果.標準出力) {
        Ok(計測) => 計測,
        Err(理由) => {
            eprintln!("[xtask] ow3-dod失敗: 統合経路の計測行を読み取れなかった: {理由}");
            return ExitCode::FAILURE;
        }
    };
    let 合格 = 計測.要約.ディスク読込件数 == 25
        && 計測.要約.最大台帳登録数 == 25
        && 計測.要約.gpu解除件数 > 0
        && 計測.要約.lod変更フレーム数 > 0
        && 計測.要約.lod変更時読込開始件数 == 0
        && 計測.要約.最終段種類数 >= 2
        && 計測.要約.最大ramバイト数 <= 上限バイト数
        && 計測.要約.最大vramバイト数 <= 上限バイト数
        && 計測.validation件数 == 0;
    if !合格 {
        eprintln!("[xtask] ow3-dod失敗: {}", 計測.要約文());
        return ExitCode::FAILURE;
    }
    let Some(png絶対パス) = run::pngへ変換する(&実行結果.ダンプ先) else {
        return ExitCode::FAILURE;
    };
    println!("[xtask] ow3-dod成功: {}", 計測.要約文());
    println!("[xtask] 複数LOD画像: {}", png絶対パス.display());
    ExitCode::SUCCESS
}

fn 子xtaskを実行する(名前: &str) -> bool {
    println!("[xtask] cargo xtask {名前} を実行");
    match Command::new("cargo").args(["xtask", 名前]).status() {
        Ok(状態) if 状態.success() => true,
        Ok(状態) => {
            eprintln!("[xtask] {名前}が{状態}で失敗した");
            false
        }
        Err(誤り) => {
            eprintln!("[xtask] {名前}を起動できなかった: {誤り}");
            false
        }
    }
}
