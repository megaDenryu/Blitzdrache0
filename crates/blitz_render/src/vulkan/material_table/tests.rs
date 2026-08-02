//! 資源表世代の状態遷移を固定する狭い検査。担当するのは、GPUの実物に依らない検査用の供給元と材質の組み立てを用意し、
//! 世代・台帳・梱包工程の検査へ配ることである。
//!
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段4aが挙げる(i)から(vi)がこの木の検査の一覧である。

mod build_failure_tests;
mod capacity_tests;
mod fallback_tests;
mod fixture;
mod generation_tests;
mod ledger_tests;
mod material_fixture;
mod packer_tests;
mod publication_tests;
mod registry_tests;
