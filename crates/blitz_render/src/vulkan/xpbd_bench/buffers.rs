//! 計測のバッファ群(判断9): 静的な引数(点・拘束の引数・隣接表2本)と、毎刻み書き換える状態(前の位置・
//! ラグランジュ乗数・方式ごとの作業域2本)と、ホスト可視の定数UBO。静的な引数と状態を同じバッファへ詰めない。
//! 方式が使わない作業域も確保するのは、ディスクリプタの宣言を方式で変えないためである。方式が使うぶんだけを
//! `方式が使うバイト数`が数え、メモリの評価軸にはそれを載せる。生成手順は`create`にある。

mod create;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::専用メモリ付きバッファ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::xpbd_solver_bench_probe::XPBD並列方式;

pub(super) struct XPBD計測バッファ {
    pub(super) 定数: 専用メモリ付きバッファ,
    pub(super) 点: 専用メモリ付きバッファ,
    pub(super) 前の位置: 専用メモリ付きバッファ,
    pub(super) 拘束の引数: 専用メモリ付きバッファ,
    pub(super) ラグランジュ乗数: 専用メモリ付きバッファ,
    pub(super) 補正の累積: 専用メモリ付きバッファ,
    pub(super) 補正の候補: 専用メモリ付きバッファ,
    pub(super) 隣接の区間: 専用メモリ付きバッファ,
    pub(super) 隣接の項目: 専用メモリ付きバッファ,
}

impl XPBD計測バッファ {
    /// 前提: 呼び出しは直前の送信の完了後(刻みごとにフェンスで待つ)。
    pub(super) fn 定数を書き込む(&self, device: &ash::Device, バイト列: &[u8]) -> Result<(), レンダラーエラー> {
        self.定数.ホスト可視の中身を書き換える(device, バイト列)
    }

    /// その方式が読み書きするバッファの確保量の合計。定数UBOは含めない(方式で変わらない)。
    pub(super) fn 方式が使うバイト数(&self, 方式: XPBD並列方式) -> u64 {
        let 共通 = [&self.点, &self.前の位置, &self.拘束の引数, &self.ラグランジュ乗数];
        let 方式の作業域: &[&専用メモリ付きバッファ] = match 方式 {
            XPBD並列方式::原子加算 => &[&self.補正の累積, &self.隣接の区間],
            XPBD並列方式::グラフ彩色 => &[],
            XPBD並列方式::二段階 => &[&self.補正の候補, &self.隣接の区間, &self.隣接の項目],
        };
        共通.iter().chain(方式の作業域.iter()).map(|バッファ| バッファ.確保バイト数()).sum()
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        for バッファ in [
            &self.定数,
            &self.点,
            &self.前の位置,
            &self.拘束の引数,
            &self.ラグランジュ乗数,
            &self.補正の累積,
            &self.補正の候補,
            &self.隣接の区間,
            &self.隣接の項目,
        ] {
            バッファ.破棄する(device);
        }
    }
}
