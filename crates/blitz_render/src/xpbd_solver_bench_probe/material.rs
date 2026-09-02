//! XPBDの並列方式の計測(Issue #35)のGPU境界型。blitz_simが`gpu_layout::xpbd`の契約で作ったバイト列と、
//! 彩色の区間と、方式の別を検証付きで保持する。バイト列のレイアウトは`shaders/xpbd_step.slang`冒頭の表と一致する
//! (点16バイト・拘束16バイト・隣接の区間4バイト×(点の数+1)・隣接の項目4バイト×(拘束の数×2))。

use thiserror::Error;

/// 比べる3つの並列方式(判断7)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XPBD並列方式 {
    原子加算,
    グラフ彩色,
    二段階,
}

/// 彩色の1色が占める、拘束の並びの中の連続した区間。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XPBD彩色の区間 {
    pub 開始: u32,
    pub 本数: u32,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum XPBD計測素材エラー {
    #[error("{名前}のバイト列長{実際}が期待{期待}と一致しない")]
    バイト列長不一致 { 名前: &'static str, 期待: usize, 実際: usize },
    #[error("彩色の区間の本数の合計{合計}が拘束の数{拘束の数}と一致しない")]
    彩色の区間が拘束を覆わない { 合計: u32, 拘束の数: u32 },
    #[error("点の数と拘束の数はどちらも1以上でなければならない(点{点の数}・拘束{拘束の数})")]
    空のグラフ { 点の数: u32, 拘束の数: u32 },
}

/// 拘束グラフ1つぶんのGPU入力。彩色の区間はグラフ彩色の方式だけが読み、他の方式では空でよい。
#[derive(Debug, Clone)]
pub struct XPBD計測素材 {
    pub(crate) 点の状態バイト列: Vec<u8>,
    pub(crate) 拘束の引数バイト列: Vec<u8>,
    pub(crate) 隣接の区間バイト列: Vec<u8>,
    pub(crate) 隣接の項目バイト列: Vec<u8>,
    pub(crate) 色の区間一覧: Vec<XPBD彩色の区間>,
    pub(crate) 点の数: u32,
    pub(crate) 拘束の数: u32,
}

impl XPBD計測素材 {
    pub fn 生成する(
        点の状態バイト列: Vec<u8>,
        拘束の引数バイト列: Vec<u8>,
        隣接の区間バイト列: Vec<u8>,
        隣接の項目バイト列: Vec<u8>,
        色の区間一覧: Vec<XPBD彩色の区間>,
        点の数: u32,
        拘束の数: u32,
    ) -> Result<Self, XPBD計測素材エラー> {
        if 点の数 == 0 || 拘束の数 == 0 {
            return Err(XPBD計測素材エラー::空のグラフ { 点の数, 拘束の数 });
        }
        let 点 = 数へ(点の数);
        let 拘束 = 数へ(拘束の数);
        長さを検証する("点の状態バイト列", 点の状態バイト列.len(), 点 * 16)?;
        長さを検証する("拘束の引数バイト列", 拘束の引数バイト列.len(), 拘束 * 16)?;
        長さを検証する("隣接の区間バイト列", 隣接の区間バイト列.len(), (点 + 1) * 4)?;
        長さを検証する("隣接の項目バイト列", 隣接の項目バイト列.len(), 拘束 * 2 * 4)?;
        let 合計: u32 = 色の区間一覧.iter().map(|区間| 区間.本数).sum();
        if !色の区間一覧.is_empty() && 合計 != 拘束の数 {
            return Err(XPBD計測素材エラー::彩色の区間が拘束を覆わない { 合計, 拘束の数 });
        }
        Ok(Self {
            点の状態バイト列,
            拘束の引数バイト列,
            隣接の区間バイト列,
            隣接の項目バイト列,
            色の区間一覧,
            点の数,
            拘束の数,
        })
    }

    pub fn 点の数(&self) -> u32 {
        self.点の数
    }

    pub fn 拘束の数(&self) -> u32 {
        self.拘束の数
    }
}

fn 長さを検証する(名前: &'static str, 実際: usize, 期待: usize) -> Result<(), XPBD計測素材エラー> {
    if 実際 != 期待 {
        return Err(XPBD計測素材エラー::バイト列長不一致 { 名前, 期待, 実際 });
    }
    Ok(())
}

fn 数へ(値: u32) -> usize {
    usize::try_from(値).unwrap_or_else(|_| panic!("数がusizeに収まらない: {値}"))
}
