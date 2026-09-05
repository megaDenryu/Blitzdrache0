//! 接触島が占める接触拘束と接触点集合の添字区間(判断17)。
//! プリミティブ執着を避け、区間の操作とスライスの切り出しを型で持つ。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断17: 接触島は動的剛体の連結成分であり、島の中の反復の順序は鍵の辞書式昇順である」

/// 接触島がバッチや接触点集合一覧の中で占める半開区間 [開始, 終了)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 島の拘束の添字区間 {
    開始: usize,
    終了: usize,
}

impl 島の拘束の添字区間 {
    /// 開始添字と終了添字から生成する。開始 > 終了 の場合は開始 = 終了として空区間にする。
    pub fn 生成する(開始: usize, 終了: usize) -> Self {
        let 正規化した終了 = 終了.max(開始);
        Self {
            開始, 終了: 正規化した終了
        }
    }

    /// 拘束を1件も含まない空の区間。
    pub fn 空() -> Self {
        Self { 開始: 0, 終了: 0 }
    }

    pub fn 開始(&self) -> usize {
        self.開始
    }

    pub fn 終了(&self) -> usize {
        self.終了
    }

    pub fn 長さ(&self) -> usize {
        self.終了.saturating_sub(self.開始)
    }

    pub fn 空か(&self) -> bool {
        self.開始 >= self.終了
    }

    /// 一覧からこの島が占めるスライスを切り出す。
    pub fn スライスを切り出す<'a, T>(&self, 一覧: &'a [T]) -> &'a [T] {
        if self.空か() || self.開始 >= 一覧.len() {
            return &[];
        }
        let 有効な終了 = self.終了.min(一覧.len());
        &一覧[self.開始..有効な終了]
    }

    /// 一覧からこの島が占める可変スライスを切り出す。
    pub fn 可変スライスを切り出す<'a, T>(&self, 一覧: &'a mut [T]) -> &'a mut [T] {
        if self.空か() || self.開始 >= 一覧.len() {
            return &mut [];
        }
        let 有効な終了 = self.終了.min(一覧.len());
        &mut 一覧[self.開始..有効な終了]
    }
}
