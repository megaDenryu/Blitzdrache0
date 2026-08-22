//! エディターが保存して焼いた実行時アセット一式を、歩行専用ゲームとストリーミング付きで開く入口。

use std::process::ExitCode;

use crate::acceptance::{
    アプリの起こし方, アプリの起動指定, 世界を読ませて報告を採る実行環境, 実行時アセットルート, 検収の実行名, 検収シーン名
};

const 実行名: 検収の実行名 = 検収の実行名::定数から生成する("editor_world_play");
const シーン名: 検収シーン名 = 検収シーン名::生成する("terrain_editor_world");
const 容量上限バイト: &str = "67108864";

pub(super) fn エディター世界を起動する() -> ExitCode {
    let 環境 = 世界を読ませて報告を採る実行環境::作る(
        アプリの起こし方::毎回cargoにリリース版を構築させて起動する,
        実行時アセットルート::綴りから生成する("target/editor_world_assets"),
    );
    let 指定 = アプリの起動指定::シーンを決めて人が終えるまで描かせる(シーン名)
        .値を持つ選択肢を足す("--game", "walk_only")
        .選択肢を足す("--streaming")
        .値を持つ選択肢を足す("--streaming-ram-limit", 容量上限バイト)
        .値を持つ選択肢を足す("--streaming-vram-limit", 容量上限バイト);
    match 環境.画面へ流したまま走らせる(実行名, &指定) {
        Ok(()) => ExitCode::SUCCESS,
        Err(誤り) => {
            eprintln!("[xtask] {誤り}");
            ExitCode::FAILURE
        }
    }
}
