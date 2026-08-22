//! 評価軸3.4節の最小ベンチ入口: 固定シーン(helmet)+固定カメラ(既定姿勢、入力なし)+
//! 固定フレーム数で`blitz_app`を実行し、パス別GPU時間とCPU側フレーム間隔分布を表示する。
//! スモークと同じ思想(繰り返す検証は資産化)で`cargo xtask bench`として登録する。
//! 参照: `_doc/計画/評価軸.md`「3.4 計測の再現性」。
//!
//! 出力を捕まえずに画面へ流すのは、600フレーム走るあいだの進み具合を人がその場で見るためである。
//! 読む世界の置き場をアプリの既定へ任せるのは、この入口が既定の世界の`helmet`だけを描くためである。

use std::process::ExitCode;

use crate::acceptance::{
    アプリの起こし方, アプリの起動指定, 世界を読ませて報告を採る実行環境, 描画フレーム数, 検収の実行名, 検収シーン名
};
use crate::fetch_assets;

const シーン名: 検収シーン名 = 検収シーン名::生成する("helmet");
const フレーム数: 描画フレーム数 = 描画フレーム数::生成する(600);
const 報告を出させる選択肢: [&str; 4] = ["--particles", "--report-gpu-times", "--report-frame-times", "--report-memory"];

/// 実表示計測を有効にするかどうか。2つの計測条件を取り違えないよう、真偽値でなく型で持つ。
///
/// 注意: `あり`は`vkWaitForPresentKHR`が表示まで描画ループを止める条件であり、拡張の有効化も伴う。
/// 既存の性能時系列(M11以降)と比較する値は`なし`で採る。両条件の比較は交互実行で行うこと。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum 実表示計測 {
    なし,
    あり,
}

impl 実表示計測 {
    /// その条件の書き出しの基準名。2つの条件の記録が同じ名前を取らないようにする。
    fn 実行名(self) -> 検収の実行名 {
        match self {
            Self::なし => 検収の実行名::定数から生成する("bench"),
            Self::あり => 検収の実行名::定数から生成する("bench_display_timing"),
        }
    }
}

pub fn 固定シーンを計測する() -> ExitCode {
    条件を指定して実行する(実表示計測::なし)
}

pub fn 実表示計測つきで実行する() -> ExitCode {
    println!("[xtask] 注意: 実表示計測は表示まで描画ループを止めるため、フレームペーシングを変えうる条件である");
    println!("[xtask] 注意: 既存の性能時系列と比較する値は`cargo xtask bench`で採ること");
    条件を指定して実行する(実表示計測::あり)
}

fn 条件を指定して実行する(実表示計測: 実表示計測) -> ExitCode {
    println!("[xtask] ベンチ用アセットの取得確認");
    if fetch_assets::標準サンプルを取得する() != ExitCode::SUCCESS {
        eprintln!("[xtask] benchのアセット取得に失敗した");
        return ExitCode::FAILURE;
    }
    if !crate::compile_assets::既定を生成する() {
        return ExitCode::FAILURE;
    }
    let 実行環境 = 世界を読ませて報告を採る実行環境::世界の置き場をアプリの既定に任せて作る(
        アプリの起こし方::毎回cargoにリリース版を構築させて起動する,
    );
    match 実行環境.画面へ流したまま走らせる(実表示計測.実行名(), &起動指定を組み立てる(実表示計測)) {
        Ok(()) => {
            println!("[xtask] bench成功");
            ExitCode::SUCCESS
        }
        Err(誤り) => {
            eprintln!("[xtask] benchが失敗した: {誤り}");
            ExitCode::FAILURE
        }
    }
}

fn 起動指定を組み立てる(実表示計測: 実表示計測) -> アプリの起動指定 {
    let 指定 = アプリの起動指定::シーンと計測の枚数を決める(シーン名, フレーム数).選択肢をまとめて足す(&報告を出させる選択肢);
    match 実表示計測 {
        実表示計測::なし => 指定,
        実表示計測::あり => 指定.選択肢を足す("--report-display-timing"),
    }
}
