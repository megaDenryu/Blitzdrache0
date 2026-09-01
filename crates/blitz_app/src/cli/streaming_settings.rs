//! チャンクストリーミングの起動設定。既存のスモークとベンチはチャンク世界を使わないため、CLIで明示的に有効化したときだけ動かす。

mod fixed_route;

use blitz_engine::{ストリーミングメモリ量, チャンク座標, チャンク読込設定};
use blitz_render::地形詳細段;

pub(crate) use fixed_route::固定経路起動設定;

/// 検証用世界で縮退を発動できる暫定値。1チャンクの見積は216バイト、読込後の実測は369バイトであり、
/// RAM上限をこの間に置くと中心チャンク1件だけを収容して外周を縮退させ、読込完了後も上限を超えない。
/// 参照: `_doc/計画/評価軸.md`
const 既定RAM上限バイト数: u64 = 400;

/// VRAMは常駐チャンクの見積を数える。先読み半径1の必要集合9件がすべて常駐しても縮退しないよう、見積9件分を上回る値を置き、縮退の観測をRAM側に集める。
const 既定VRAM上限バイト数: u64 = 4096;
/// プレイヤーの大域位置をどこから得るか。位置の出どころで読込順の再現性が変わるため、真偽値でなく選択肢として持つ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum プレイヤー位置源 {
    カメラ視点,                 // 既定。経路を明示しない指定であり、中心にする位置は遊ぶゲームで決まる(`app/streaming/route.rs`)
    固定経路,                   // --streaming-route指定。フレーム番号から決まる固定経路を使い、実行のたびに同じ位置列を得る
    Ow3Dod経路,                 // cargo xtask ow3-dod専用。中心から始めて半径2を全件先読みし、静止後に境界とLOD閾値を横切る
    インスタンスストリーム経路, // cargo xtask instance-stream専用。Ow3Dod経路と同じ先読みと横断のあと、始点まで戻って静止し整定する
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LOD継ぎ目検査設定 {
    pub(crate) 一方座標: チャンク座標,
    pub(crate) 一方段: 地形詳細段,
    pub(crate) 他方座標: チャンク座標,
    pub(crate) 他方段: 地形詳細段,
    pub(crate) 欠落座標: Option<チャンク座標>,
}

/// チャンクストリーミングの起動設定一式。
///
/// - `要約を報告する`: `--report-streaming-summary`は毎フレームの順序出力(`報告する`)とは別の指定にする。順序出力が事象のあるフレームごとに複数行を出し、長時間実行では標準出力量が桁違いになるためである。
pub(crate) struct ストリーミング起動設定 {
    pub(crate) 有効: bool, // --streaming指定でtrue。既定はfalse(既存のスモーク8ステージとベンチの挙動を1つも変えない)
    pub(crate) 上限: ストリーミングメモリ量,
    pub(crate) 報告する: bool,       // --report-streaming指定でtrue。読込・解除の順を標準出力へ出す
    pub(crate) 要約を報告する: bool, // --report-streaming-summary指定でtrue。終了時に転送量・処理時間分布・最大使用量を出す
    pub(crate) 位置源: プレイヤー位置源,
    pub(crate) lod継ぎ目検査: Option<LOD継ぎ目検査設定>,
    pub(crate) 先読み半径: u8,
    pub(crate) 固定経路: 固定経路起動設定,
    pub(crate) 読込: チャンク読込設定,
}

impl ストリーミング起動設定 {
    pub(crate) fn 既定値() -> Self {
        Self {
            有効: false,
            上限: ストリーミングメモリ量::生成する(既定RAM上限バイト数, 既定VRAM上限バイト数),
            報告する: false,
            要約を報告する: false,
            位置源: プレイヤー位置源::カメラ視点,
            lod継ぎ目検査: None,
            先読み半径: 1,
            固定経路: 固定経路起動設定::既定値(),
            読込: チャンク読込設定::既定値(),
        }
    }
}
