//! 1つのLOD組合せぶんの起動の指定。受け取るのは検査条件、返すのは実行環境へ渡す起動指定である。
//! 起こす手順そのものは検収の共通語彙の実行環境が持つ。
//!
//! 地形世界は空を持つ方針であるため、番兵背景の露出を数えるこの検査は空パスを明示的に外す。
//! 時間再構成は`--no-taa`で外す。この入口の判定がバイト一致に依るため、フレームをまたぐ混合が入ると前のフレームの残りが絵に混ざる。

use std::path::PathBuf;

use crate::acceptance::{
    アプリの起こし方, アプリの起動指定, 実行時アセットルート, 描画フレーム数, 描画検収の実行環境, 検収エラー, 検収シーン名
};

use super::cases::検査条件;

const アセットルート: &str = "target/terrain_assets";
const シーン名: 検収シーン名 = 検収シーン名::生成する("terrain_origin");
const フレーム数: 描画フレーム数 = 描画フレーム数::生成する(120);
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const 背景と光を外す選択肢: [&str; 4] = ["--unlit", "--no-post", "--no-sky", "--no-taa"];

pub(super) fn 実行環境を作る(出力ディレクトリ: PathBuf) -> Result<描画検収の実行環境, 検収エラー> {
    描画検収の実行環境::作る(
        アプリの起こし方::毎回cargoに構築させて起動する,
        実行時アセットルート::綴りから生成する(アセットルート),
        出力ディレクトリ,
    )
}

pub(super) fn 起動指定を組み立てる(条件: &検査条件) -> アプリの起動指定 {
    let (x1, z1) = 条件.一方;
    let (x2, z2) = 条件.他方;
    let 組の綴り = [
        x1.to_string(),
        z1.to_string(),
        条件.一方段.to_string(),
        x2.to_string(),
        z2.to_string(),
        条件.他方段.to_string(),
    ];
    let 指定 = アプリの起動指定::シーンと枚数を決める(シーン名, フレーム数)
        .選択肢を足す("--streaming")
        .値を持つ選択肢を足す("--streaming-preload-radius", 先読み半径)
        .値を持つ選択肢を足す("--streaming-ram-limit", 容量上限バイト)
        .値を持つ選択肢を足す("--streaming-vram-limit", 容量上限バイト)
        .選択肢をまとめて足す(&背景と光を外す選択肢)
        .選択肢を足す("--lod-crack-pair")
        .選択肢をまとめて足す(&組の綴り.each_ref().map(String::as_str));
    let Some((欠落x, 欠落z)) = 条件.欠落 else {
        return 指定;
    };
    指定
        .選択肢を足す("--lod-crack-missing")
        .選択肢をまとめて足す(&[欠落x.to_string().as_str(), 欠落z.to_string().as_str()])
}
