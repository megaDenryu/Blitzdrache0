//! 採取条件から、固定構図を描かせるアプリの起動指定を組む工程。受け取るのは採取条件、返すのは起動指定である。
//!
//! 構図を決める選択肢は全部の採取で同じであり、条件ごとに変わるのは立てる旗だけである。
//! 構図の側をここ1箇所に持つのは、採取ごとに書くと1つの採取だけ別の構図で撮った絵を突き合わせるためである。

use crate::acceptance::{アプリの起動指定, 描画フレーム数};
use crate::distant_view::plan::採取条件;

const 枚数: 描画フレーム数 = 描画フレーム数::生成する(360);

pub(super) fn 起動指定(条件: &採取条件) -> アプリの起動指定 {
    let mut 指定 = アプリの起動指定::シーンと枚数を決める(crate::fox_tour_launch::シーン名, 枚数)
        .選択肢を足す("--streaming")
        .値を持つ選択肢を足す("--streaming-preload-radius", "8")
        .値を持つ選択肢を足す("--streaming-ram-limit", "536870912")
        .値を持つ選択肢を足す("--streaming-vram-limit", "536870912")
        .値を持つ選択肢を足す("--streaming-loader-workers", "1")
        .値を持つ選択肢を足す("--game", "fox_tour")
        .値を持つ選択肢を足す("--camera-yaw", "180")
        .値を持つ選択肢を足す("--camera-pitch", "-11.0107")
        .値を持つ選択肢を足す("--time-of-day", "43200")
        .選択肢を足す("--no-taa")
        .選択肢を足す("--no-auto-exposure")
        .値を持つ選択肢を足す("--depth-prepass", "equal");
    if 条件.ssaoを使わない {
        指定 = 指定.選択肢を足す("--no-ssao");
    }
    if 条件.遠景影を使わない {
        指定 = 指定.選択肢を足す("--no-distant-shadow");
    }
    if 条件.後処理を使わない {
        指定 = 指定.選択肢を足す("--no-post");
    }
    if 条件.影可視度を可視化する {
        指定 = 指定.選択肢を足す("--debug-shadow-loss");
    }
    if 条件.明示境界を使わない {
        指定 = 指定.選択肢を足す("--no-explicit-shadow-bands");
    }
    指定
}
