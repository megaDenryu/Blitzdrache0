//! 個体別LODの選択の検査。初回の床関数・境界値ちょうど・ヒステリシス帯の内側での往復・段数上限での飽和・
//! 個体ごとの独立・切替回数の数え方を確かめる。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「個体別LOD」

#![allow(clippy::unwrap_used)]

mod boundary;
mod fixture;
mod independence;
