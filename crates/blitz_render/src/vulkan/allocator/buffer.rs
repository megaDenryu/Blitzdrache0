//! バッファの確保。担当する工程は「バッファを作り、要件に合う専用メモリを結び付け、途中で失敗したら
//! その場で作りかけを片付ける」ことであり、受け取るのはバイト数と用途、返すのは専用メモリ付きバッファである。

use ash::vk;

use super::memory_type::メモリの置き場;
use super::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;

impl GPU資源の確保係<'_> {
    /// GPUの中だけで読み書きするバッファを確保する。中身は呼び出し側が転送で書くまで未定義である。
    pub(crate) fn デバイスローカルバッファを確保する(
        &self,
        バイト数: u64,
        用途: vk::BufferUsageFlags,
    ) -> Result<専用メモリ付きバッファ, レンダラーエラー> {
        self.バッファを作って専用メモリを結び付ける(
            バイト数,
            用途,
            メモリの置き場::デバイスローカル,
            GPUメモリ用途::デバイスバッファ,
        )
    }

    /// ホストから書けるバッファを確保し、渡したバイト列で初期化する。ステージングとシェーダー定数が使う。
    pub(crate) fn ホスト可視バッファを確保して書き込む(
        &self,
        データ: &[u8],
        用途: vk::BufferUsageFlags,
    ) -> Result<専用メモリ付きバッファ, レンダラーエラー> {
        let バイト数 = u64::try_from(データ.len()).unwrap_or_else(|_| panic!("バッファサイズがu64に収まらない"));
        let 確保済み = self.バッファを作って専用メモリを結び付ける(
            バイト数,
            用途,
            メモリの置き場::ホスト可視,
            GPUメモリ用途::ホストバッファ,
        )?;
        if let Err(誤り) = 確保済み.ホスト可視の中身を書き換える(self.device, データ) {
            確保済み.破棄する(self.device);
            return Err(誤り);
        }
        Ok(確保済み)
    }

    /// GPUが書いた結果をホストへ持ち帰るためのバッファを確保する。中身は転送で埋まるため初期化しない。
    pub(crate) fn 読み戻し先のホスト可視バッファを確保する(
        &self,
        バイト数: u64,
        用途: vk::BufferUsageFlags,
    ) -> Result<専用メモリ付きバッファ, レンダラーエラー> {
        self.バッファを作って専用メモリを結び付ける(バイト数, 用途, メモリの置き場::ホスト可視, GPUメモリ用途::読み戻しバッファ)
    }

    fn バッファを作って専用メモリを結び付ける(
        &self,
        バイト数: u64,
        用途: vk::BufferUsageFlags,
        置き場: メモリの置き場,
        メモリ用途: GPUメモリ用途,
    ) -> Result<専用メモリ付きバッファ, レンダラーエラー> {
        let create_info = vk::BufferCreateInfo::default()
            .size(バイト数)
            .usage(用途)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        // 安全性: deviceは生成済みで有効。
        let buffer = unsafe { self.device.create_buffer(&create_info, None)? };
        // 安全性: bufferは直前に生成済み。
        let 要件 = unsafe { self.device.get_buffer_memory_requirements(buffer) };
        let memory = match self.要件に合う専用メモリを確保する(要件, 置き場, メモリ用途) {
            Ok(memory) => memory,
            Err(誤り) => {
                // 安全性: bufferはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { self.device.destroy_buffer(buffer, None) };
                return Err(誤り);
            }
        };
        // 安全性: bufferとmemoryはともに直前に生成済みで、offsetは0(専用確保のため衝突しない)。
        if let Err(誤り) = unsafe { self.device.bind_buffer_memory(buffer, memory, 0) } {
            // 安全性: bufferはこのスコープの唯一の所有者で、結び付けに失敗したためGPUは使用していない。
            unsafe { self.device.destroy_buffer(buffer, None) };
            self.device.メモリを解放する(memory);
            return Err(誤り.into());
        }
        Ok(専用メモリ付きバッファ::組にする(buffer, memory, 要件.size))
    }
}
