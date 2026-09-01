//! 起動した子プロセスの木の番号を集める台帳。担当するのは、通常の終了とCtrl+Cの割り込みという
//! 2つの経路から、同じ木の集合を終わらせられる状態を保つことである。
//!
//! `Child`でなく番号を持つのは、割り込みの処理が別のスレッドで走り、`Child`を共有できないためである。

use std::sync::Mutex;

use super::process_id::プロセス番号;

pub(crate) struct 停止台帳 {
    番号一覧: Mutex<Vec<プロセス番号>>,
}

impl 停止台帳 {
    pub(crate) fn 空で作る() -> Self {
        Self {
            番号一覧: Mutex::new(Vec::new()),
        }
    }

    pub(crate) fn 木を登録する(&self, 番号: プロセス番号) {
        match self.番号一覧.lock() {
            Ok(mut 一覧) => 一覧.push(番号),
            Err(毒された錠) => 毒された錠.into_inner().push(番号),
        }
    }

    /// 注意: 錠が毒されていても中身を取り出して進める。ここで諦めると、後始末そのものが飛んで
    /// 子プロセスが残る。後始末の失敗は待ち受け口の占拠として次回の起動に響くため、必ず通す。
    pub(crate) fn 登録済みの木を全て終わらせる(&self) {
        let 一覧 = match self.番号一覧.lock() {
            Ok(掴んだ一覧) => 掴んだ一覧.clone(),
            Err(毒された錠) => 毒された錠.into_inner().clone(),
        };
        for 番号 in 一覧 {
            番号.この番号を根とする木を終わらせる();
        }
    }
}
