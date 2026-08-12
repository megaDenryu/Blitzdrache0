//! グラフ実行器: パス列を宣言順に走査し、バリアを発行してからパスを記録する。
//! バリア発行の呼び出しはここ(`barrier_apply::発行する`経由)に集約する
//! （参照: `_doc/設計/レンダーグラフ.md`「M5のDoD対応」）。パス前後のGPU
//! タイムスタンプ書き込み(判断30)もここに集約し、呼び出し元へ「パス名→クエリ開始添字」
//! の対応を返す。
//!
//! 積み先と計測のクエリプールを構築時に受けて保持するのは、走査の各工程へこの2つを引数で運ばせないためである。

mod pass_record;

use ash::vk;

use super::barrier_apply;
use super::barrier_derivation::{self, 地点別バリア};
use super::buffer_barrier_derivation;
use super::builder::グラフ;
use super::pass_resource_usage::パスリソース使用;
use super::timestamp_write::{クエリ開始添字を求める, タイムスタンプを書く};
use crate::vulkan::command_sink::GPU命令の積み先;

pub(crate) struct グラフ実行器<'デバイス> {
    積み先: GPU命令の積み先<'デバイス>,
    /// 計測が有効なフレームだけ`Some`。`Some`ならパス前後にタイムスタンプを書き、パス名とクエリ開始添字の対応を返す。
    クエリプール: Option<vk::QueryPool>,
}

impl<'デバイス> グラフ実行器<'デバイス> {
    pub(crate) fn 生成する(積み先: GPU命令の積み先<'デバイス>, クエリプール: Option<vk::QueryPool>) -> Self {
        Self {
            積み先, クエリプール
        }
    }

    /// グラフを実行し、コマンドバッファへ記録する。グラフはここで消費される。
    /// 返るのはパス名とクエリ開始添字の対応であり、計測無効の実行では空配列になる。
    pub(crate) fn グラフを積む(&self, グラフ: グラフ<'_>) -> Vec<(&'static str, u32)> {
        let (画像レジストリ, バッファレジストリ, 画像初期状態表, バッファ初期状態表, 最終用途表, パス列) = グラフ.分解する();

        let リソース使用列: Vec<パスリソース使用> = パス列.iter().map(パスリソース使用::from).collect();
        let 地点別画像バリア列 = barrier_derivation::導出する(&画像初期状態表, &リソース使用列, &最終用途表);
        let パス別バッファバリア列 = buffer_barrier_derivation::導出する(&バッファ初期状態表, &リソース使用列);

        let mut 地点別画像バリア列 = 地点別画像バリア列.into_iter();
        let mut 計測マッピング = Vec::new();
        for (パス添字, パス) in パス列.into_iter().enumerate() {
            let 地点別: 地点別バリア = 地点別画像バリア列
                .next()
                .unwrap_or_else(|| panic!("パス数とバリア導出結果の数が一致しない(実装のバグ)"));
            let バッファバリア一覧 = パス別バッファバリア列
                .get(パス添字)
                .unwrap_or_else(|| panic!("パス数とバッファバリア導出結果の数が一致しない(実装のバグ)"));
            barrier_apply::発行する(self.積み先, &画像レジストリ, &バッファレジストリ, &地点別.バリア一覧, バッファバリア一覧);

            let パス名 = パス.名前;
            let クエリ開始添字 = クエリ開始添字を求める(パス添字);
            if let Some(pool) = self.クエリプール {
                タイムスタンプを書く(self.積み先, pool, クエリ開始添字);
                計測マッピング.push((パス名, クエリ開始添字));
            }
            pass_record::パスを記録する(self.積み先, &画像レジストリ, &バッファレジストリ, パス);
            if let Some(pool) = self.クエリプール {
                タイムスタンプを書く(self.積み先, pool, クエリ開始添字 + 1);
            }
        }

        let 終端: 地点別バリア = 地点別画像バリア列
            .next()
            .unwrap_or_else(|| panic!("グラフ終端のバリア導出結果が欠けている(実装のバグ)"));
        barrier_apply::発行する(self.積み先, &画像レジストリ, &バッファレジストリ, &終端.バリア一覧, &[]);

        計測マッピング
    }
}
