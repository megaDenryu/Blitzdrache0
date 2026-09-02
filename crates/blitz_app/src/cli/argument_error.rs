//! CLI引数の解析で返す型付きエラー。どの引数がどう不正だったかの語彙を所有する。
//!
//! 大半の枝は引数の値そのものが不正な場合を表すが、`局所可視性補正が深度プリパスを要る`と
//! `時間再構成がポスト処理を要る`の2つは、引数そのものは読めたが世界の宣言と組み合わせられない場合を表す。
//! - `フレームダンプ不正`: 引数名は`--dump-frame`と`--dump-hdr-frame`の2つがあるため、どちらだったかは中身が名指しする。
//! - `局所可視性補正が深度プリパスを要る`: 局所可視性補正は深度プリパスが書いた深度を読むため、プリパスを積まない指定と重ねると1画素も効かない絵になる。
//! - `時間再構成がポスト処理を要る`: 時間再構成の結果はHDR中間画像へ書くため、ポスト処理を組まない構成では書き先が無い。無言で使わない側へ落とすと、地形世界を`--no-post`で開いた実行だけ絵の作り方が黙って変わる。

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum 起動引数エラー {
    #[error("--frames引数が不正だった: {0}")]
    フレーム数不正(String),
    #[error("--shader-source引数が不正だった: {0}")]
    シェーダーソース不正(String),
    #[error("--scene引数が不正だった: {0}")]
    シーン名不正(String),
    #[error("--game引数が不正だった: {0}")]
    遊ぶゲーム不正(String),
    #[error("--asset-root引数が不正だった: {0}")]
    アセットルート不正(String),
    #[error("--object-count引数が不正だった: {0}")]
    描画対象数不正(String),
    #[error("フレームダンプの引数が不正だった: {0}")]
    フレームダンプ不正(String), // --dump-frame/--dump-hdr-frameのどちらだったかは中身が名指しする
    #[error("--exposure引数が不正だった: {0}")]
    露出不正(String),
    #[error("--lod-probe-step引数が不正だった: {0}")]
    LOD探査刻み不正(String),
    #[error("--local-light-count引数が不正だった: {0}")]
    局所光の件数不正(String),
    #[error("--point-light-shadow-count引数が不正だった: {0}")]
    影を落とす灯の件数不正(String),
    #[error("--blend引数が不正だった: {0}")]
    ブレンド不正(String),
    #[error("--cloth-xpbd-reference引数が不正だった: {0}")]
    布のコンプライアンス不正(String),
    #[error("ストリーミング上限の引数が不正だった: {0}")]
    ストリーミング上限不正(String),
    #[error("--global-offset引数が不正だった: {0}")]
    大域ずらし量不正(String),
    #[error("--camera-nudge引数が不正だった: {0}")]
    カメラずれ不正(String),
    #[error("--camera-pitch引数が不正だった: {0}")]
    カメラ俯角不正(String),
    #[error("--camera-yaw引数が不正だった: {0}")]
    カメラ方位不正(String),
    #[error("--time-of-day引数が不正だった: {0}")]
    一日内時刻不正(String),
    #[error("--time-scale引数が不正だった: {0}")]
    時間倍率不正(String),
    #[error("--lod-crack-pair引数が不正だった: {0}")]
    Lod継ぎ目検査不正(String),
    #[error("--streaming-preload-radius引数が不正だった: {0}")]
    先読み半径不正(String),
    #[error("固定経路の引数が不正だった: {0}")]
    固定経路不正(String),
    #[error("チャンク読込設定の引数が不正だった: {0}")]
    チャンク読込設定不正(String),
    #[error("--indirect-probe引数が不正だった: {0}")]
    遠方環境の検収条件不正(String),
    #[error("XPBDの並列方式の計測の引数が不正だった: {0}")]
    XPBD並列方式計測の指定不正(String),

    #[error("--auto-exposure-probe引数が不正だった: {0}")]
    自動露出の探り色不正(String),
    #[error("--report-sky-pixel引数が不正だった: {0}")]
    空代表画素不正(String),
    #[error("--shadow-resolution引数が不正だった: {0}")]
    影の一辺解像度不正(String),
    #[error("--caster-margin引数が不正だった: {0}")]
    キャスター余白不正(String),
    #[error("--max-shadow-distance引数が不正だった: {0}")]
    最大影距離不正(String),
    #[error("--shadow-caster-range引数が不正だった: {0}")]
    影の視距離不正(String),
    #[error("--ibl-step-scan引数が不正だった: {0}")]
    段差走査範囲不正(String),
    #[error(
        "局所可視性補正を宣言した世界へ--depth-prepass noneを指定した。この組み合わせでは深度が消去値のままになり、局所可視性補正が1画素も効かない"
    )]
    局所可視性補正が深度プリパスを要る, // 引数は読めたが世界の宣言と組み合わせられない場合(深度プリパスの深度を読むため)
    #[error("--depth-prepass引数が不正だった: {0}")]
    深度プリパス方式不正(String),
    #[error("局所可視性補正の検収の指定が不正だった: {0}")]
    局所可視性の検収指定不正(String),
    #[error(
        "時間再構成を宣言した世界へ--no-postを指定した。この組み合わせでは再構成の結果を書くHDR中間画像が無い。--no-taaを重ねて時間再構成を切ること"
    )]
    時間再構成がポスト処理を要る, // 引数は読めたが世界の宣言と組み合わせられない場合(結果を書くHDR中間画像が無いため)
}
