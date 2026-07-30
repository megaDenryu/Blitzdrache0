//! 開発用UIテクスチャの安定ID。呼び出し側(blitz_app)がegui側のテクスチャIDから
//! 決定的に写像して生成する(このクレートはeguiを知らない)。

/// UIテクスチャの登録/更新/削除を参照するための識別子(newtype)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UIテクスチャID(u64);

impl UIテクスチャID {
    /// 呼び出し側が一意に管理する値からIDを生成する。
    pub fn 生成する(値: u64) -> Self {
        Self(値)
    }
}
