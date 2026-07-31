//! レンダラーの明示的な破棄処理。フィールドのDrop順序に頼らず、
//! 生成の逆順で全Vulkanリソースを破棄する:
//! device_wait_idle → 読み戻しバッファ・パイプライン・コマンド・ジオメトリ → 提示資源
//! → GPU環境(デバイス → サーフェス → メッセンジャー → インスタンス)。

use super::レンダラー;

impl レンダラー {
    pub(super) fn 破棄する(&mut self) {
        // 前提: この呼び出し以降このレンダラーは使用されない(Dropが唯一の呼び出し元)。
        // 破棄処理には失敗の伝播先が無いため、待機の失敗を捨てるのはこの1箇所だけである(集約側は`Result`を返す)。
        let _ = self.環境.gpuの全作業完了を待つ();
        self.任意資源を破棄する();
        self.描画資源を破棄する();
        self.基盤を破棄する();
    }

    fn 任意資源を破棄する(&self) {
        let device = self.環境.device();
        if let Some(バッファ) = &self.読み戻しバッファ {
            バッファ.破棄する(device);
        }
        self.ui一式.破棄する(device);
        if let Some(粒子) = &self.粒子 {
            粒子.破棄する(device);
        }
        if let Some(計測) = &self.gpu計測 {
            計測.破棄する(device);
        }
        self.描画段階資源.破棄する(device);
        if let Some(スキニング) = &self.スキニング {
            スキニング.破棄する(device);
        }
        if let Some(布) = &self.布 {
            布.破棄する(device);
        }
        if let Some(ポスト処理) = &self.ポスト処理 {
            ポスト処理.破棄する(device);
        }
    }

    fn 描画資源を破棄する(&self) {
        let device = self.環境.device();
        self.フレーム進行.破棄する(device);
        self.シーン描画資源.破棄する(device);
        self.シェーダー定数.破棄する(device);
        self.転送環境.破棄する(device);
        self.シャドウマップ.破棄する(device);
    }

    /// 提示資源の破棄と専用メモリの全解放確認は、GPU環境がvkDestroyDeviceを呼ぶ前でなければ
    /// 意味を持たないため、環境の破棄の外に明示的に置く。
    ///
    /// 注意: 提示資源は深度バッファの専用メモリを持つため、`全メモリ解放を確認する`より前に破棄する。
    /// この確認はレンダラー全体の破棄順を持つこの箇所の責任であり、提示資源の束の中へは入れない。
    fn 基盤を破棄する(&self) {
        self.提示.破棄する(&self.環境);
        self.環境.device().全メモリ解放を確認する();
        self.環境.破棄する();
    }
}
