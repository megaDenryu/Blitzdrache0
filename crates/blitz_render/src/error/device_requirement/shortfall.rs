//! 1つの物理デバイスが索引化の最低要件のうち何を欠くかという、機材1件ぶんの不足内訳。
//! 機材名を一緒に持つのは、複数のGPUを積んだ機材で「どのGPUを見て何を直せばよいか」が
//! 不足機能の並びだけでは決まらないためである。`機材名`を読み取れなかった機材では、その旨を述べる文字列が入る。

use std::fmt;

use super::ディスクリプタ索引機能項目;

#[derive(Debug)]
pub struct 機材別のディスクリプタ索引機能不足 {
    機材名: String,                            // `VkPhysicalDeviceProperties::deviceName`
    不足一覧: Vec<ディスクリプタ索引機能項目>, // 欠けている機能。空にならないことを生成側(選定)が保つ
}

impl 機材別のディスクリプタ索引機能不足 {
    pub(crate) fn 生成する(機材名: String, 不足一覧: Vec<ディスクリプタ索引機能項目>) -> Self {
        Self { 機材名, 不足一覧 }
    }

    pub fn 機材名(&self) -> &str {
        &self.機材名
    }

    pub fn 不足一覧(&self) -> &[ディスクリプタ索引機能項目] {
        &self.不足一覧
    }
}

impl fmt::Display for 機材別のディスクリプタ索引機能不足 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let 機能名一覧: Vec<&str> = self.不足一覧.iter().map(|項目| 項目.機能名()).collect();
        write!(f, "{}(不足: {})", self.機材名, 機能名一覧.join(", "))
    }
}
