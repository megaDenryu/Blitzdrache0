//! 骨格方式の試験と、試験が使う材料の束。実データを写した接合点・カタログ・候補・規則と、それらを使う4つの試験を持つ。
//!
//! 展開の層の直下から1つの束へまとめるのは、層の入口が個々の試験の名前を並べる場所ではないためである。
//! 骨格方式は接合点の材料が4つ・規則の材料が3つ・試験が6つあり、家と木の試験を合わせた数より多い。
//!
//! 束の中の分け方は、材料が「何を写したものか」で分かれ、試験が「何を確かめるか」で分かれる。
//! 材料は骨格自身の接合点・取り付く部品の接合点・煙突の接合点・カタログ・候補・1ベイ1段の規則・連なりの規則・
//! 飾りの8つ、試験は据わる位置・連なり・屋根・床・飾り・煙突の6つである。

#![cfg(test)]

mod frame_attached_joint_fixture;
mod frame_bay_joint_fixture;
mod frame_chain_choice_fixture;
mod frame_chain_fixture;
mod frame_chain_rule_fixture;
mod frame_chain_tests;
mod frame_chimney_joint_fixture;
mod frame_chimney_tests;
mod frame_fixture;
mod frame_floor_tests;
mod frame_ornament_fixture;
mod frame_ornament_tests;
mod frame_placement_tests;
mod frame_roof_tests;
mod frame_rule_fixture;
mod frame_rule_tests;
