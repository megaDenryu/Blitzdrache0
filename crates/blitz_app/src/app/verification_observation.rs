//! 検収の観測。フレーム間隔の計測・可視個体の選別の計測・シーン読込の計数・時間再構成の前フレームの結果・
//! スモークの基準画像の5つは、実行を通して積み上がる観測であり、1つの型が持つ。命令は記録する系と
//! 終了時の統計を作ることであり、どれも検収の実行だけが読む。本番のフレーム経路が変えるのはフレーム間隔の記録だけである。
//! 参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断11」。
//!
//! 起動時にレンダラーの計測を有効にする局面は`renderer_measurement`、終了時の統計は`summary`が持つ。

mod renderer_measurement;
mod scene_read_count;
mod section_timing;
mod summary;

pub(crate) use summary::検収の観測の統計;

use std::time::Duration;

use blitz_render::{HDR読み戻し画像, 読み戻し画像};

use super::frame_timing::フレーム間隔計測;
use super::time_step::フレーム番号;
use crate::cli::起動設定;
use scene_read_count::シーン読込計数;
use section_timing::区間計測;

pub(in crate::app) struct 検収の観測 {
    フレーム間隔計測: Option<フレーム間隔計測>,            // `--report-frame-times`指定時だけ、ウォームアップ後のフレーム開始間隔を貯める
    可視個体の選別の計測: Option<区間計測>,                // `--report-instance-sections`指定時だけ1フレーム分の走査時間を貯める
    シーン読込計数: シーン読込計数,                        // ディスクから実行時シーンを読んだ回数
    時間再構成の前フレームの結果: Option<HDR読み戻し画像>, // `--report-temporal-reconstruction`指定の実行だけが使う、前のフレームの再構成結果1枚
    スモーク基準画像: Option<読み戻し画像>,                // foxステージの差分判定が比べる基準フレームの読み戻し
}

impl 検収の観測 {
    pub(in crate::app) fn 起動設定から作る(設定: &起動設定) -> Self {
        Self {
            フレーム間隔計測: 設定.フレーム時間報告.then(|| フレーム間隔計測::生成する(設定.モード)),
            可視個体の選別の計測: 設定.インスタンス区間報告.then(|| 区間計測::生成する(設定.モード)),
            シーン読込計数: シーン読込計数::default(),
            時間再構成の前フレームの結果: None,
            スモーク基準画像: None,
        }
    }

    /// 描画機会の先頭で呼ぶ。計測が無効な実行では時刻を1度も読まない。
    pub(in crate::app) fn フレーム間隔を記録する(&mut self) {
        if let Some(計測) = &mut self.フレーム間隔計測 {
            計測.記録する();
        }
    }

    /// フレーム間隔を測る実行か。`--report-frame-times`が指定されたときだけ真である。
    fn フレーム間隔を測るか(&self) -> bool {
        self.フレーム間隔計測.is_some()
    }

    /// 可視個体の選別の時間を測る実行か。測らない実行で時刻を読むと、既存の性能時系列の条件が指定なしで変わる。
    pub(in crate::app) fn 選別の時間を測るか(&self) -> bool {
        self.可視個体の選別の計測.is_some()
    }

    pub(in crate::app) fn 選別の時間を貯める(&mut self, 所要時間: Duration) {
        if let Some(計測) = &mut self.可視個体の選別の計測 {
            計測.記録する(所要時間);
        }
    }

    /// 実行時シーンをディスクから読んだ直後に呼ぶ。
    pub(in crate::app) fn ディスクから読んだことを数える(&mut self, フレーム番号: フレーム番号) {
        self.シーン読込計数.読み込んだ(フレーム番号);
    }

    pub(in crate::app) fn 時間再構成の前フレームの結果を見る(&self) -> Option<&HDR読み戻し画像> {
        self.時間再構成の前フレームの結果.as_ref()
    }

    /// 今のフレームの再構成結果を据え、次のフレームの差分の相手にする。
    pub(in crate::app) fn 時間再構成の一枚を据える(&mut self, 結果: HDR読み戻し画像) {
        self.時間再構成の前フレームの結果 = Some(結果);
    }

    pub(in crate::app) fn 基準画像を据える(&mut self, 画像: 読み戻し画像) {
        self.スモーク基準画像 = Some(画像);
    }

    pub(in crate::app) fn 基準画像を見る(&self) -> Option<&読み戻し画像> {
        self.スモーク基準画像.as_ref()
    }
}
