//! blitz_render 全体で使う型付きエラー。
//!
//! 参照: CLAUDE.md「エラー・パニック」。Vulkanの実行時失敗は握り潰さずここへ集約し、
//! 呼び出し元へ `?` で伝播する。
//!
//! 注意: `From<ash::LoadingError>` / `From<ash::vk::Result>` はash型を「受け取って
//! 自前表現へ変換する」だけであり、公開APIからash型を取り出せる経路にはならない。
//! バリアント自体は自前表現（String・Vulkan失敗コード）のみを保持する。

use thiserror::Error;

use crate::vulkan_failure::Vulkan失敗コード;

/// レンダラーの生成・描画・破棄で起こりうる失敗を表す層のエラー型。
#[derive(Debug, Error)]
pub enum レンダラーエラー {
    /// Vulkanローダー(vulkan-1.dll)の動的読み込みに失敗した。
    #[error("Vulkanローダーの読み込みに失敗した: {0}")]
    ローダー読み込み失敗(String),

    /// Vulkan API呼び出しがエラーコードを返した。
    #[error("Vulkan呼び出しが失敗した: {0}")]
    Vulkan呼び出し失敗(Vulkan失敗コード),

    /// グラフィックス描画・提示、および必須機能(dynamicRendering・synchronization2・
    /// shaderDrawParameters)の両方に対応する物理デバイスが1つも見つからなかった。
    #[error(
        "グラフィックス表示・提示、および必須機能(dynamicRendering/synchronization2/shaderDrawParameters)に対応する物理デバイスが見つからなかった"
    )]
    適合物理デバイスなし,

    /// サーフェスが提示可能な形式を1つも報告しなかった（Vulkan仕様上は
    /// 到達しないはずの経路だが、外部デバイス由来のため型で防ぎきれない）。
    #[error("サーフェスが提示形式を1つも報告しなかった")]
    サーフェス形式なし,

    /// SPIR-Vバイト列の解釈（マジックナンバー確認・u32語への変換）に失敗した。
    /// `シェーダー一式` は4バイト整列を検証済みのため、ここに到達するのは
    /// slangcが不正なSPIR-Vを出力した場合のみ。
    #[error("SPIR-Vバイト列の読み込みに失敗した: {0}")]
    SPIRV読み込み失敗(String),

    /// ホスト可視かつコヒーレントなメモリ型が物理デバイスに1つも無かった
    /// （読み戻しバッファの確保に必要。Vulkan仕様上は到達しないはずの経路）。
    #[error("ホスト可視かつコヒーレントなメモリ型が見つからなかった")]
    ホスト可視メモリ型なし,

    /// サーフェスがTRANSFER_SRC用途をサポートせず、`一フレーム描画して読み戻す`が
    /// 実行できない（判断9: ほぼ全環境で対応するが、非対応環境では通常描画のみ可能）。
    #[error("サーフェスがTRANSFER_SRC用途をサポートしないため読み戻せない")]
    読み戻し非対応,

    /// デバイスローカルなメモリ型が物理デバイスに1つも無かった（深度バッファの確保に必要。
    /// Vulkan仕様上は到達しないはずの経路）。
    #[error("デバイスローカルなメモリ型が見つからなかった")]
    デバイスローカルメモリ型なし,
}

impl From<ash::LoadingError> for レンダラーエラー {
    fn from(誤り: ash::LoadingError) -> Self {
        Self::ローダー読み込み失敗(誤り.to_string())
    }
}

impl From<ash::vk::Result> for レンダラーエラー {
    fn from(結果: ash::vk::Result) -> Self {
        Self::Vulkan呼び出し失敗(Vulkan失敗コード::生成する(結果))
    }
}
