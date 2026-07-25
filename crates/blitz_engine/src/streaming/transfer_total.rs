//! 起動から現在までにストリーミングが運んだ量の累計。ディスクからCPUへ読んだ量、CPUからGPUへ載せた量、GPUから解除した件数を数える。
//! 使用量が台帳の状態から導出できるのに対し、この累計は過ぎ去った事象の合計であり状態からは導出できないため、値として持ち回る。
//! 加算は飽和も折り返しもせず、u64を超えたら値なしを返して呼出し側に型付きエラーを作らせる。

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ストリーミング転送量 {
    ディスク読込件数: u64,
    ディスク読込バイト数: u64,
    gpu転送件数: u64,
    gpu転送バイト数: u64,
    gpu解除件数: u64,
}

impl ストリーミング転送量 {
    /// 読込ワーカーが完了を返した回数。必要でなくなって捨てた分も、ディスクを読んだ事実として数える。
    pub fn ディスク読込件数(self) -> u64 {
        self.ディスク読込件数
    }

    /// 読込ワーカーが返した実測バイト数の合計。版付きファイル全体の長さである。
    pub fn ディスク読込バイト数(self) -> u64 {
        self.ディスク読込バイト数
    }

    pub fn gpu転送件数(self) -> u64 {
        self.gpu転送件数
    }

    /// GPUへ載せた見積VRAMバイト数の合計。VRAMは実測を持たないため見積を積む。
    pub fn gpu転送バイト数(self) -> u64 {
        self.gpu転送バイト数
    }

    pub fn gpu解除件数(self) -> u64 {
        self.gpu解除件数
    }

    pub(super) fn ディスク読込を足す(self, バイト数: u64) -> Option<Self> {
        Some(Self {
            ディスク読込件数: self.ディスク読込件数.checked_add(1)?,
            ディスク読込バイト数: self.ディスク読込バイト数.checked_add(バイト数)?,
            ..self
        })
    }

    pub(super) fn gpu転送を足す(self, バイト数: u64) -> Option<Self> {
        Some(Self {
            gpu転送件数: self.gpu転送件数.checked_add(1)?,
            gpu転送バイト数: self.gpu転送バイト数.checked_add(バイト数)?,
            ..self
        })
    }

    pub(super) fn gpu解除を足す(self) -> Option<Self> {
        Some(Self {
            gpu解除件数: self.gpu解除件数.checked_add(1)?,
            ..self
        })
    }
}
