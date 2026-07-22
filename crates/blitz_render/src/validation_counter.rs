//! validation layerが検出したエラー・警告件数を数える、複製可能な値オブジェクト。

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// debug utilsメッセンジャーのコールバックから加算される、エラー・警告の合計件数。
/// clone しても同じ実体（同一のAtomicU64）を指す。
///
/// 注意: 破棄後（レンダラーDrop後）に読むこと。破棄処理自体が発するvalidation
/// メッセージも件数に含めるため、読み取りの前にレンダラーの全Vulkanリソースが
/// 破棄済みである必要がある。
#[derive(Debug, Clone)]
pub struct 検証カウンタ {
    件数: Arc<AtomicU64>,
}

impl 検証カウンタ {
    /// ゼロで初期化したカウンタを生成する。
    pub(crate) fn 生成する() -> Self {
        Self {
            件数: Arc::new(AtomicU64::new(0)),
        }
    }

    /// debug utilsメッセンジャーのuser_dataに渡すため、内部Arcを複製して返す。
    /// 加算自体はコールバックが生ポインタ越しにAtomicU64へ直接行うため、
    /// この型自身は加算メソッドを持たない。
    /// 呼び出し側（デバッグメッセンジャー構造体）がこの複製を保持し続けることで、
    /// メッセンジャー破棄までポインタの指す先が生存することを保証する。
    pub(crate) fn 内部参照を複製する(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.件数)
    }

    /// 現在までの合計件数を読む。
    pub fn 件数を読む(&self) -> u64 {
        self.件数.load(Ordering::SeqCst)
    }
}
