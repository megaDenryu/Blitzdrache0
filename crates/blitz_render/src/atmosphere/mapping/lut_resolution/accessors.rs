//! ベイク済み画像の寸法と要素数を読み出す入口。

use super::大気のベイク済み画像の解像度;

impl 大気のベイク済み画像の解像度 {
    pub fn 透過率の幅(&self) -> u32 {
        self.透過率の幅
    }
    pub fn 透過率の高さ(&self) -> u32 {
        self.透過率の高さ
    }
    pub fn 多重散乱の一辺(&self) -> u32 {
        self.多重散乱の一辺
    }
    pub fn スカイビューの幅(&self) -> u32 {
        self.スカイビューの幅
    }
    pub fn スカイビューの高さ(&self) -> u32 {
        self.スカイビューの高さ
    }
    pub fn 空中遠近の幅(&self) -> u32 {
        self.空中遠近の幅
    }
    pub fn 空中遠近の高さ(&self) -> u32 {
        self.空中遠近の高さ
    }
    pub fn 空中遠近の奥行き(&self) -> u32 {
        self.空中遠近の奥行き
    }

    pub fn 透過率のテクセル数(&self) -> usize {
        要素数(&[self.透過率の幅, self.透過率の高さ])
    }
    pub fn 多重散乱のテクセル数(&self) -> usize {
        要素数(&[self.多重散乱の一辺, self.多重散乱の一辺])
    }
    pub fn スカイビューのテクセル数(&self) -> usize {
        要素数(&[self.スカイビューの幅, self.スカイビューの高さ])
    }
    pub fn 空中遠近のボクセル数(&self) -> usize {
        要素数(&[self.空中遠近の幅, self.空中遠近の高さ, self.空中遠近の奥行き])
    }
}

fn 要素数(辺一覧: &[u32]) -> usize {
    辺一覧.iter().fold(1_usize, |積, 辺| {
        let 長さ = usize::try_from(*辺).unwrap_or_else(|_| panic!("ベイク済み画像の辺の長さ{辺}がusizeに収まらない"));
        積 * 長さ
    })
}

#[cfg(test)]
mod tests {
    use super::大気のベイク済み画像の解像度;

    #[test]
    fn 既定の空中遠近は三十二掛ける三十二掛ける百六十() {
        let 解像度 = 大気のベイク済み画像の解像度::既定値();
        assert_eq!([解像度.空中遠近の幅(), 解像度.空中遠近の高さ(), 解像度.空中遠近の奥行き()], [32, 32, 160]);
        assert_eq!(解像度.空中遠近のボクセル数(), 163_840);
    }
}
