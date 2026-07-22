//! フレーム番号を進め、固定フレーム実行の終了条件を適用する。

use winit::event_loop::ActiveEventLoop;

use super::super::アプリ;
use crate::cli::起動モード;

pub(super) fn 進めて必要なら終了する(アプリ: &mut アプリ, event_loop: &ActiveEventLoop) {
    アプリ.現在フレーム += 1;
    let 終了フレーム数 = match アプリ.起動モード {
        起動モード::スモーク実行 { フレーム数 } | 起動モード::ベンチ実行 { フレーム数 } => Some(フレーム数),
        起動モード::無期限実行 => None,
    };
    if 終了フレーム数.is_some_and(|フレーム数| アプリ.現在フレーム >= フレーム数) {
        event_loop.exit();
    }
}
