//! XPBDの並列方式の計測一式(Issue #35): バッファ群・ディスクリプタ・方式ごとのコンピュートパイプラインと、
//! ウィンドウを持たないGPUでの進行。本番の布の経路(`vulkan::cloth`)とは資源もパスも共有しない。
//! 生成の局面はこのファイル、パスの積み方は`passes`、進行と入口は`run`、GPU時間は`timing`、読み戻しは`readback`が持つ。

mod buffers;
mod descriptor;
pub(crate) mod params;
pub(crate) mod pass_names;
mod passes;
mod pipelines;
mod readback;
mod run;
mod timing;

use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::ステージング経由の転送係;
use crate::xpbd_solver_bench_probe::XPBDシェーダー一式;
use crate::xpbd_solver_bench_probe::XPBD計測の条件;
use crate::xpbd_solver_bench_probe::{XPBD彩色の区間, XPBD計測素材};

pub(crate) use run::走らせて読み戻す;

pub(crate) struct XPBD計測一式 {
    バッファ: buffers::XPBD計測バッファ,
    ディスクリプタ: descriptor::XPBD計測ディスクリプタ,
    パイプライン群: pipelines::XPBD計測パイプライン群,
    色の区間一覧: Vec<XPBD彩色の区間>,
    点の数: u32,
    拘束の数: u32,
}

impl XPBD計測一式 {
    fn 生成する(
        転送係: ステージング経由の転送係<'_>,
        素材: &XPBD計測素材,
        条件: &XPBD計測の条件,
        シェーダー: &XPBDシェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let device = 転送係.論理デバイス();
        let バッファ = buffers::XPBD計測バッファ::生成する(転送係, 素材)?;
        let ディスクリプタ = match descriptor::XPBD計測ディスクリプタ::生成する(device, &バッファ) {
            Ok(一式) => 一式,
            Err(誤り) => {
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        let パイプライン群 = match pipelines::XPBD計測パイプライン群::生成する(
            転送係.確保係(),
            ディスクリプタ.レイアウトのハンドル(),
            シェーダー,
            条件.方式,
        ) {
            Ok(群) => 群,
            Err(誤り) => {
                ディスクリプタ.破棄する(device);
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            バッファ,
            ディスクリプタ,
            パイプライン群,
            色の区間一覧: 素材.色の区間一覧.clone(),
            点の数: 素材.点の数,
            拘束の数: 素材.拘束の数,
        })
    }

    fn 破棄する(&self, device: &GPUデバイス) {
        self.パイプライン群.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.バッファ.破棄する(device);
    }
}
