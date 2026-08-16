//! どのチャンク世界を1つの出力ルートへ焼くかの選択。1つの出力ルートは1つのカタログと1つのチャンク目録を持つため、
//! 同じ座標を持つ2つの世界は同じ出力ルートへ同居できない。世界の選択がそのまま出力ルートの選択になる。
//! どのアセットを焼くかの宣言は`asset_declaration`が、定義1件の組み立ては`definition_kind`が持ち、
//! プロセス境界の綴りとその解析は`argument_name`が、世界のソースディレクトリ名は`directory_source_path`が、
//! チャンクのソース形式は`chunk_source_kind`が、地面へ散らす原型の一覧は`scatter_declaration`が持つ。宣言をコンパイラが受け取る指定へ写す手順は、
//! 小物群を`prop_group_declaration`が、目視見本を`visual_sample_declaration`が、地面へ据える固定物を`fixed_placement_declaration`が持ち、
//! 世界ごとの宣言は`fox_tour_declaration`と`stone_hut_declaration`と`night_lights_declaration`と`part_house_row_declaration`と
//! `part_tree_row_declaration`にあり、部品で組んだ並びの宣言の型と規則は`part_row_declaration`が持つ。

mod argument_name;
pub(super) mod assembled_scatter_declaration;
mod assembly_rule_choice;
mod asset_declaration;
mod asset_definition_list;
mod chunk_source_kind;
mod definition_kind;
mod directory_source_path;
pub(super) mod fixed_placement_declaration;
pub(super) mod fox_tour_declaration;
mod fox_tour_scatter_declaration;
mod night_lights_declaration;
pub(super) mod part_house_row_declaration;
pub(super) mod part_row_declaration;
mod part_tree_row_declaration;
pub(super) mod prop_group_declaration;
mod provenance;
mod scatter_declaration;
pub(super) mod stone_hut_declaration;
mod vegetation_declaration;
mod vertex_diagnostic_declaration;
mod village_declaration;
pub(super) mod visual_sample_declaration;

use blitz_asset_compiler::{アセット配置エラー, 世界のディレクトリ名, 散布の焼き方};

use super::catalog::{アセット定義, ソース種別};
use part_house_row_declaration::家の並びの規模;
use vertex_diagnostic_declaration::診断の原型;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 対象世界 {
    板の世界,
    地形の世界,
    植生の世界,
    見本の集落の世界,
    /// 部品を組み合わせて家を建てる世界。同じ地面へ規模だけが違う2つがあり、発行数が件数に依らないことをこの対で見る。
    /// 参照: `_doc/設計/部品カタログと接合点.md`「段の計画」
    部品で組んだ家の並びの世界(家の並びの規模),
    /// 部品を組み合わせて樫の木を組み、平らな地面へ数本並べる世界。組み上がった木の姿を人が近くで見るためのものであり、
    /// 計器の実証は家の並びが持つ。参照: `_doc/設計/部品カタログと接合点.md`「段5の到達点」
    部品で組んだ木の並びの世界,
    /// 間接照明の絵をオーナーが目で確かめるための世界。地面の上へ材質見本の立体と少数の小物を据える。
    目視見本の世界,
    /// 頂点処理量の係数を同定するための計測専用の世界。地形の代表世界と同じ地面と配置を持ち、同居植生の原型のトポロジー量だけが違う。
    頂点診断の世界(診断の原型),
    /// ブロック圧縮の絵と誤差の統計を確かめるための世界。対照の素材をベースカラーに持つ板2枚を原点チャンクへ置き、チャンク以外に焼くものを持たない。参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`「判断i」
    ブロック圧縮の対照世界,
    /// 夜の多光源の検収世界。起伏のある地面1チャンクと、影の形で灯を見分けるための遮蔽物2件を持つ。点光源はblitz_appの世界の宣言が置く。
    /// 参照: `_doc/設計/クラスタ多光源と点光源の影.md`「検収戦略(判断i)」
    夜の多光源の世界,
    /// 屋内の多光源の検収世界。平らな地面1チャンクの中央へ石積みの小屋を1棟据える。
    屋内の多光源の世界,
    /// クソゲー1本目「キツネの場所巡り」を遊ぶ世界。乱数の種から生成した9チャンクの地面へ目的地の目印を立て、
    /// 起動時に読むキツネのシーンを一緒に焼く。参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「第1段階の定義」
    場所巡りの世界,
}

impl 対象世界 {
    pub(super) fn 引数名から解析する(引数名: &str) -> Result<Self, String> {
        argument_name::解析する(引数名)
    }

    pub(super) fn 世界のソースディレクトリ名(self) -> Result<世界のディレクトリ名, アセット配置エラー> {
        directory_source_path::世界のソースディレクトリ名を選ぶ(self)
    }

    /// その世界のチャンクがどのソース形式で書かれているか。台帳は`chunk_source_kind`が持つ。
    pub(super) fn チャンクのソース種別(self, 同居植生個体数: usize, 散布: 散布の焼き方) -> ソース種別 {
        chunk_source_kind::チャンクのソース種別を選ぶ(self, 同居植生個体数, 散布)
    }

    /// 同居植生の個体数を指定されなかったときに使う値。地形と頂点診断以外の世界は同居植生を持たないため、この値を読まない。
    pub(super) fn 同居植生の既定個体数() -> usize {
        asset_declaration::地形同居の既定個体数
    }

    /// その世界が高さ場アセットを焼くか。実行中に地形の高さを参照するのは場所巡りの世界だけであり、
    /// 検収の世界は高さ場を持たない(持たせると既存の世界のカタログの中身が変わる)。
    pub(super) fn 高さ場を焼くか(self) -> bool {
        self == Self::場所巡りの世界
    }

    /// チャンク以外に焼く、この世界のアセット一覧。台帳は`asset_definition_list`が持つ。
    pub(super) fn アセット定義一覧(self) -> Vec<アセット定義> {
        asset_definition_list::アセット定義一覧を選ぶ(self)
    }
}
