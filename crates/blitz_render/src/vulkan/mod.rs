//! Vulkan(ash) の unsafe をこの階層の実装内部に封じ込める内部モジュール群。
//! ここに置く型・関数はすべて `pub(crate)`。上位公開APIは親クレートの各ファイルが担う。
//!
//! 参照: CLAUDE.md「unsafe の規律」

pub(crate) mod bloom;
pub(crate) mod bloom_targets;
pub(crate) mod cloth;
pub(crate) mod commands;
pub(crate) mod compute_pipeline;
pub(crate) mod debug_messenger;
pub(crate) mod depth;
pub(crate) mod descriptor;
pub(crate) mod device;
pub(crate) mod device_buffer;
pub(crate) mod frame;
pub(crate) mod fullscreen_pipeline;
pub(crate) mod geometry;
pub(crate) mod gpu_timing;
pub(crate) mod graph;
pub(crate) mod hdr_target;
pub(crate) mod host_buffer;
pub(crate) mod instance;
pub(crate) mod linear_sampler;
pub(crate) mod memory;
pub(crate) mod memory_ledger;
pub(crate) mod particles;
pub(crate) mod physical_device;
pub(crate) mod pipeline;
pub(crate) mod readback;
pub(crate) mod shader_module;
pub(crate) mod shadow_map;
pub(crate) mod skinning;
pub(crate) mod surface;
pub(crate) mod swapchain;
pub(crate) mod sync;
pub(crate) mod texture;
pub(crate) mod tonemap;
pub(crate) mod tracked_device;
pub(crate) mod transfer;
pub(crate) mod ui;
pub(crate) mod uniform;
