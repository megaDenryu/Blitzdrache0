//! 埋め込んだSPIR-Vそのものへ掛ける検査の束。原文の修飾が最終SPIR-Vまで残るかは原文からは分からないため、
//! 生成物を読んで確かめる検査をここに集める。
//!
//! 現在2つある。材質テクスチャ表への参照が非一様として装飾されること(`nonuniform`。段4bの検収ゲート(i))と、
//! 自動露出の単精度の四則が融合しないものとして装飾されること(`no_contraction`)である。
//! どちらもSPIR-Vの読み解き(`spirv_module`)を共有するため、1つの親の下に置く。

mod no_contraction;
mod nonuniform;
mod nonuniform_scan;
mod spirv_module;
