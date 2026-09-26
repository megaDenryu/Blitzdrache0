// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/地点と経路/図式.rs:25
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    188105804555868394u64, 11071486581159505625u64, 5811251637526133488u64,
    16597272256562868644u64,
];
/// `地点` ノードの公開ID。
///
/// 宣言: `src/地点と経路/図式.rs` の `node 地点`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 地点Id(pub String);
/// `経路` 辺の公開ID。
///
/// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 経路Id(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __地点InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __経路InternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __地点NamedPosition(__地点InternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __経路NamedPosition(__経路InternalPosition, u64);
/// 構築時に組み立てる `経路` 辺の値。
///
/// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
pub struct 経路 {
    endpoints: graphite::UnorderedPair<地点Id>,
    /// この辺が運ぶ積み荷。
    pub 予定: 経路の開閉の予定,
}
impl 経路 {
    /// 両端の公開IDと積み荷から構築用の辺値を作る。両端の順序は保たない。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn new(a: 地点Id, b: 地点Id, payload: 経路の開閉の予定) -> Self {
        Self {
            endpoints: graphite::UnorderedPair::new(a, b),
            予定: payload,
        }
    }
    /// この辺値の両端の公開IDを順序なし対として借用する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn endpoints(&self) -> (&地点Id, &地点Id) {
        self.endpoints.endpoints()
    }
    /// この辺値が運ぶ積み荷を借用する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn payload(&self) -> &経路の開閉の予定 {
        &self.予定
    }
}
impl graphite::UndirectedEdgeLiteral<地点Id, 経路の開閉の予定> for 経路 {
    fn from_graph_literal(
        a: 地点Id,
        b: 地点Id,
        payload: 経路の開閉の予定,
    ) -> Self {
        Self::new(a, b, payload)
    }
}
impl std::fmt::Debug for 経路 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(経路))
    }
}
#[allow(dead_code)]
#[derive(Clone)]
struct __経路Record {
    endpoints: graphite::UnorderedPair<__地点InternalPosition>,
    予定: 経路の開閉の予定,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `src/地点と経路/図式.rs` の `schema 地点と経路の図式`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    Duplicate地点(地点Id),
    /// このエッジ種別のキーが重複している。
    経路DuplicateKey(経路Id),
    /// このエッジが未知の端点キーを参照している (無向のため位置の
    /// 区別は無い)。
    経路UnknownEndpoint {
        /// 未知のキーを参照した辺の公開ID。
        edge: 経路Id,
        /// この辺が端点として参照した、対応するノードが存在しないキー。
        endpoint: 地点Id,
    },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::Duplicate地点(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "地点", id)
            }
            Violation::経路DuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "経路", id)
            }
            Violation::経路UnknownEndpoint { edge, endpoint } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として見つかりません (辺 `{}` {:?} の{})",
                    endpoint, "地点", "経路", edge, "端点"
                )
            }
        }
    }
}
impl std::fmt::Debug for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl std::error::Error for Violation {}
/// 凍結済み図式グラフ。構築後の構造は不変で、ノード値と辺の積み荷だけを
/// `&mut Graph` を要求する種別APIから更新できる。
///
/// 宣言: `src/地点と経路/図式.rs` の `schema 地点と経路の図式`
#[derive(Clone)]
pub struct Graph {
    __graphite_node_地点: graphite::KeyedTable<地点Id, super::地点>,
    経路: graphite::KeyedTable<経路Id, __経路Record>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    経路_index: graphite::MultipleRoleIndex<__経路InternalPosition>,
    __graphite_経路_by_pair: std::collections::HashMap<
        graphite::UnorderedPair<__地点InternalPosition>,
        Vec<__経路InternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `node 地点`
    pub fn 地点_by_id<'graph>(
        &'graph self,
        id: &地点Id,
    ) -> Option<地点Ref<'graph>> {
        let internal_position = __地点InternalPosition(
            self.__graphite_node_地点.position(id)?,
        );
        Some(地点Ref {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `node 地点`
    pub fn 地点_value_mut(&mut self, id: &地点Id) -> Option<&mut super::地点> {
        self.__graphite_node_地点.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `node 地点`
    pub fn 地点_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph 地点Id> {
        self.__graphite_node_地点.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `node 地点`
    pub fn 地点_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 地点Ref<'graph>> + 'graph {
        self.__graphite_node_地点
            .positions()
            .map(move |position| 地点Ref {
                graph: self,
                internal_position: __地点InternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `node 地点`
    pub fn 地点_len(&self) -> usize {
        self.__graphite_node_地点.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 経路_by_id<'graph>(
        &'graph self,
        id: &経路Id,
    ) -> Option<経路Ref<'graph>> {
        Some(経路Ref {
            graph: self,
            internal_position: __経路InternalPosition(self.経路.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 経路_payload_mut(
        &mut self,
        id: &経路Id,
    ) -> Option<&mut 経路の開閉の予定> {
        self.経路.get_mut(id).map(|record: &mut __経路Record| &mut record.予定)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 経路_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph 経路Id> {
        self.経路.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 経路_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 経路Ref<'graph>> + 'graph {
        self.経路
            .positions()
            .map(move |position| 経路Ref {
                graph: self,
                internal_position: __経路InternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 経路_len(&self) -> usize {
        self.経路.len()
    }
    /// builder をクロージャに貸し出し、戻ったら凍結して図式適合
    /// (端点種別・where 制約) を一括検査する。最初の1件の違反で
    /// `Err` になる (複数の違反を全件見たい場合は
    /// [`Self::create_collecting`] を使う)。
    pub fn create<F>(f: F) -> Result<Self, Violation>
    where
        F: for<'b> FnOnce(&'b mut Builder),
    {
        let mut builder = Builder::new();
        f(&mut builder);
        builder.freeze()
    }
    /// `graph!` が名前付き要素の名前付き位置を凍結境界の外へ運ぶための
    /// 内部構築経路。`Graph` の凍結に成功した場合だけ名前付き位置を返す。
    /// [`graphite::build_named_graph`] へ薄く委譲するだけで、
    /// [`graphite::NamedInsertPermit`] はそちらでしか作らない
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn create_named<F, N>(f: F) -> Result<(Self, N), Violation>
    where
        F: for<'b> FnOnce(&'b mut Builder, &'b graphite::NamedInsertPermit) -> N,
    {
        graphite::build_named_graph(Builder::new, f)
    }
    /// [`Self::create`] の複数違反収集版。builder をクロージャに
    /// 貸し出し、戻ったら凍結して図式適合を検査する点は `create` と
    /// 同じだが、最初の1件で打ち切らず全違反を `Vec` に集めて返す。
    pub fn create_collecting<F>(f: F) -> Result<Self, Vec<Violation>>
    where
        F: for<'b> FnOnce(&'b mut Builder),
    {
        let mut builder = Builder::new();
        f(&mut builder);
        builder.freeze_collecting()
    }
}
/// 完成済みグラフ上の無向辺個体。
///
/// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
#[derive(Clone, Copy)]
pub struct 経路Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __経路InternalPosition,
}
impl<'graph> 経路Ref<'graph> {
    fn record(self) -> &'graph __経路Record {
        let Some((_, record)) = self.graph.経路.get_at(self.internal_position.0) else {
            panic!(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)"
            )
        };
        record
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn id(self) -> &'graph 経路Id {
        let Some((id, _)) = self.graph.経路.get_at(self.internal_position.0) else {
            panic!(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)"
            )
        };
        id
    }
    /// この辺個体の両端を順序なし対として返す。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn endpoints(self) -> (地点Ref<'graph>, 地点Ref<'graph>) {
        let (first, second) = self.record().endpoints.endpoints();
        (
            地点Ref {
                graph: self.graph,
                internal_position: __地点InternalPosition(first.0),
            },
            地点Ref {
                graph: self.graph,
                internal_position: __地点InternalPosition(second.0),
            },
        )
    }
    /// この辺個体が運ぶ積み荷を役割名で借用する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 予定(self) -> &'graph 経路の開閉の予定 {
        &self.record().予定
    }
    /// この辺個体が運ぶ積み荷を、役割名によらない固定名で借用する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn payload(self) -> &'graph 経路の開閉の予定 {
        &self.record().予定
    }
}
impl<'graph> std::fmt::Debug for 経路Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(経路Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
/// 制約検査を一切行わない。
///
/// 宣言: `src/地点と経路/図式.rs` の `schema 地点と経路の図式`
pub struct Builder {
    __graphite_node_地点: Vec<(地点Id, super::地点)>,
    経路: Vec<(経路Id, 経路)>,
    /// この構築を識別する構築印。`Builder::new()` が発行し、この
    /// `Builder` から挿入する全ての名前付き位置と、凍結成功後の
    /// `Graph` へ同じ値を刻む。
    __graphite_construction_stamp: u64,
}
/// 型付き ID を受け取るノード・エッジ共通の挿入トレイト。
///
/// 署名が `insert_with_id(self, b, id)` と、挿入される値を receiver に
/// して `Builder` を引数で受ける向きなのは、`graph!` がノード項の値の
/// 型を解析せず、正しい内部ストレージへの振り分けを値の型の trait
/// ディスパッチに頼るためである。利用者向けの公開入口は
/// `Builder::insert`/`Builder::add` の側にある。
///
/// `insert_named_with_id` は [`graphite::NamedInsertPermit`] を要求する
/// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
/// `insert_with_id` (許可証不要、名前付き位置を返さない) は独立した
/// 実装を持ち、`insert_named_with_id` を経由しない
/// (`create` のクロージャから許可証なしで呼べる必要があるため)。
pub trait 地点と経路の図式Insertable: Sized {
    /// この要素を挿入したときに受け取る公開ID型。
    type Id;
    #[doc(hidden)]
    type NamedPosition;
    #[doc(hidden)]
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    /// 型付きの公開IDを指定して、この要素を `Builder` へ挿入する。
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id;
}
/// 束縛名の文字列からスキーマ内限定の既定IDを作れる要素だけが
/// 実装する。明示ID型には実装せず、文字列変換を要求しない。
pub trait 地点と経路の図式DefaultId: 地点と経路の図式Insertable {
    #[doc(hidden)]
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    /// 束縛名の文字列から既定IDを作り、この要素を `Builder` へ挿入する。
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id;
}
/// ノード挿入で使うトレイト境界。読み取りは `Graph` の種別メソッドと
/// `NodeRef` のメソッドが提供する。利用者がこのトレイトのメソッドを
/// 直接呼ぶことは想定しない。
pub trait 地点と経路の図式Node: 地点と経路の図式Insertable {}
impl 地点と経路の図式Insertable for super::地点 {
    type Id = 地点Id;
    type NamedPosition = __地点NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __地点NamedPosition(
            __地点InternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_地点.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.地点(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.地点(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __地点NamedPosition {
    type Reference<'graph> = 地点Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        地点Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 地点と経路の図式DefaultId for super::地点 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        地点と経路の図式Insertable::insert_named_with_id(
            self,
            b,
            地点Id(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        地点と経路の図式Insertable::insert_with_id(self, b, 地点Id(binding))
    }
}
impl 地点と経路の図式Node for super::地点 {}
/// 完成済みグラフ上の `地点` ノード個体。
///
/// 宣言: `src/地点と経路/図式.rs` の `node 地点`
#[derive(Clone, Copy)]
pub struct 地点Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __地点InternalPosition,
}
impl<'graph> 地点Ref<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `node 地点`
    pub fn id(self) -> &'graph 地点Id {
        let Some((id, _)) = self
            .graph
            .__graphite_node_地点
            .get_at(self.internal_position.0) else {
            panic!(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)"
            )
        };
        id
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `node 地点`
    pub fn value(self) -> &'graph super::地点 {
        let Some((_, value)) = self
            .graph
            .__graphite_node_地点
            .get_at(self.internal_position.0) else {
            panic!(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)"
            )
        };
        value
    }
    /// 接続辺を O(1) で参照し、追加確保なしで挿入順に走査する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 経路_incident(self) -> impl Iterator<Item = 経路Ref<'graph>> + 'graph {
        let positions = self.graph.経路_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 経路Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序なし端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 経路_try_between(
        self,
        other: 地点Ref<'graph>,
    ) -> Result<
        impl Iterator<Item = 経路Ref<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_経路_by_pair
            .get(
                &graphite::UnorderedPair::new(
                    self.internal_position,
                    other.internal_position,
                ),
            )
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| 経路Ref {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::経路_try_between`] を使う。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 経路_between(
        self,
        other: 地点Ref<'graph>,
    ) -> impl Iterator<Item = 経路Ref<'graph>> + 'graph {
        self.経路_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(地点Ref), stringify!(経路_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for 地点Ref<'graph> {
    type Target = super::地点;
    fn deref(&self) -> &Self::Target {
        let Some((_, value)) = self
            .graph
            .__graphite_node_地点
            .get_at(self.internal_position.0) else {
            panic!(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)"
            )
        };
        value
    }
}
impl<'graph> std::fmt::Debug for 地点Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(地点Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait 地点と経路の図式Edge: 地点と経路の図式Insertable {}
impl 地点と経路の図式Insertable for 経路 {
    type Id = 経路Id;
    type NamedPosition = __経路NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __経路NamedPosition(
            __経路InternalPosition(
                graphite::TablePosition::from_index(b.経路.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.経路(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.経路(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __経路NamedPosition {
    type Reference<'graph> = 経路Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        経路Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 地点と経路の図式DefaultId for 経路 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        地点と経路の図式Insertable::insert_named_with_id(
            self,
            b,
            経路Id(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        地点と経路の図式Insertable::insert_with_id(self, b, 経路Id(binding))
    }
}
impl 地点と経路の図式Edge for 経路 {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_地点: Vec::new(),
            経路: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `node 地点`
    pub fn 地点(&mut self, id: 地点Id, value: super::地点) -> &mut Self {
        self.__graphite_node_地点.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/地点と経路/図式.rs` の `edge 経路 = 地点 -[予定: 経路の開閉の予定]- 地点`
    pub fn 経路(&mut self, id: 経路Id, value: 経路) -> &mut Self {
        self.経路.push((id, value));
        self
    }
    /// 型名付きメソッド (`b.#accessor(id, value)` 群、上記
    /// `#node_methods`) の総称版。`graph!` の左辺名付きノード項は
    /// 下記 `insert_named` (名前付き位置を返す許可証付き経路) へ
    /// 脱糖するため、このメソッド自体は `graph!` を経由しない。
    /// 値の型を手書きで組み立てる場合 (プログラム的構築など) に使う。
    /// `graph!` はノード項の値の型を一切パースしないため
    /// (`key = 式` の「式」でしかない)、値の型 (`N: #node_trait_ident`)
    /// から正しい内部ストレージへの振り分けを rustc の型推論任せに
    /// する点は `insert_named` と共通。命名判断・trait の形は
    /// `gen_node_trait_and_impls` のドキュメントコメント参照。
    pub fn insert<N>(&mut self, key: impl Into<String>, value: N) -> N::Id
    where
        N: 地点と経路の図式Node + 地点と経路の図式DefaultId,
    {
        value.insert_with_binding(self, key.into())
    }
    /// `graph!` が公開IDと名前付き要素の内部位置を同時に受け取る経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named<N>(
        &mut self,
        key: impl Into<String>,
        value: N,
        permit: &graphite::NamedInsertPermit,
    ) -> (N::Id, N::NamedPosition)
    where
        N: 地点と経路の図式Node + 地点と経路の図式DefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: 地点と経路の図式Node>(
        &mut self,
        id: N::Id,
        value: N,
    ) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: 地点と経路の図式Node>(
        &mut self,
        id: N::Id,
        value: N,
        permit: &graphite::NamedInsertPermit,
    ) -> (N::Id, N::NamedPosition) {
        value.insert_named_with_id(self, id, permit)
    }
    /// `insert` のエッジ版。`graph!` の辺行 `key = Kind(from -> to)`
    /// は名前付きフィールドの辺値型を関連コンストラクタで構築したあと、
    /// 下記 `add_named` へ脱糖する (`docs/schema_v4.md` §2/§3.2)。
    /// このメソッド自体は値の型から内部ストレージへ振り分ける総称
    /// ディスパッチを提供する手書き用APIで、`graph!` を直接経由しない。
    pub fn add<E>(&mut self, key: impl Into<String>, value: E) -> E::Id
    where
        E: 地点と経路の図式Edge + 地点と経路の図式DefaultId,
    {
        value.insert_with_binding(self, key.into())
    }
    /// `graph!` が公開IDと名前付き辺の内部位置を同時に受け取る経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named<E>(
        &mut self,
        key: impl Into<String>,
        value: E,
        permit: &graphite::NamedInsertPermit,
    ) -> (E::Id, E::NamedPosition)
    where
        E: 地点と経路の図式Edge + 地点と経路の図式DefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: 地点と経路の図式Edge>(
        &mut self,
        id: E::Id,
        value: E,
    ) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: 地点と経路の図式Edge>(
        &mut self,
        id: E::Id,
        value: E,
        permit: &graphite::NamedInsertPermit,
    ) -> (E::Id, E::NamedPosition) {
        value.insert_named_with_id(self, id, permit)
    }
    /// `insert`/`add` のイテレータ版 (`docs/bulk_construction.md`、
    /// `docs/graph_splice.md` §2)。実行時データからの構築で for
    /// ループが構築コードに残るのを避けるため、要素単位 API の反復に
    /// 完全に一致する意味論 (挿入順保持・検証は凍結時) をまとめて
    /// 提供する。ノード用・エッジ用の呼び分けが要らない単一の総称
    /// メソッドに統一している (v4 破壊的変更、旧 `extend_nodes`/
    /// `extend_edges` は廃止): 値の型が既定IDを生成できれば
    /// ノードでもエッジでもよい (どちらになるかは rustc の
    /// 型推論任せ)。`graph!` のスプライス項 (`..式`) もこのメソッドへ
    /// 脱糖する。`insert`/`add` と同じ理由 (トレイトが schema ごとに
    /// 名前が異なる) で、graphite ランタイム側の共通機構ではなく
    /// ここに生成する。
    pub fn extend<K, T>(&mut self, items: impl IntoIterator<Item = (K, T)>) -> Vec<T::Id>
    where
        K: Into<String>,
        T: 地点と経路の図式DefaultId,
    {
        items.into_iter().map(|(k, v)| v.insert_with_binding(self, k.into())).collect()
    }
    /// 検証ロジックの実体。最初の1件で打ち切らず全違反を `Vec` に
    /// 集めて返す。`freeze()` (単一エラー版) はこちらに委譲し先頭の1件を
    /// 取り出すだけの薄いラッパーにすることで、検証ロジックが二重実装に
    /// ならないようにしている。
    fn freeze_collecting(self) -> Result<Graph, Vec<Violation>> {
        let mut __violations: Vec<Violation> = Vec::new();
        let __graphite_construction_stamp = self.__graphite_construction_stamp;
        let mut __graphite_node_地点: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_地点 {
            if !__graphite_node_地点.insert(id.clone(), value) {
                __violations.push(Violation::Duplicate地点(id));
            }
        }
        let mut __graphite_経路: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut 経路_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_経路_by_pair: std::collections::HashMap<
            graphite::UnorderedPair<__地点InternalPosition>,
            Vec<__経路InternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.経路 {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::経路DuplicateKey(id));
                continue;
            }
            let 経路 { endpoints, 予定 } = value;
            let (p0, p1) = endpoints.endpoints();
            let p0 = p0.clone();
            let p1 = p1.clone();
            let first_position = __graphite_node_地点
                .position(&p0)
                .map(__地点InternalPosition);
            let second_position = __graphite_node_地点
                .position(&p1)
                .map(__地点InternalPosition);
            if first_position.is_none() {
                __violations
                    .push(Violation::経路UnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p0.clone(),
                    });
            }
            if p1 != p0 && second_position.is_none() {
                __violations
                    .push(Violation::経路UnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p1.clone(),
                    });
            }
            if let (Some(first_position), Some(second_position)) = (
                first_position,
                second_position,
            ) {
                let internal_edge_position = __経路InternalPosition(
                    graphite::TablePosition::from_index(__graphite_経路.len()),
                );
                __graphite_経路_by_pair
                    .entry(graphite::UnorderedPair::new(first_position, second_position))
                    .or_default()
                    .push(internal_edge_position);
                経路_index
                    .entry(first_position)
                    .or_default()
                    .push(internal_edge_position);
                if second_position != first_position {
                    経路_index
                        .entry(second_position)
                        .or_default()
                        .push(internal_edge_position);
                }
                let inserted = __graphite_経路
                    .insert(
                        id,
                        __経路Record {
                            endpoints: graphite::UnorderedPair::new(
                                first_position,
                                second_position,
                            ),
                            予定,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let 経路_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_地点
                .positions()
                .map(|position| {
                    経路_index
                        .remove(&__地点InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_地点,
            経路: __graphite_経路,
            経路_index,
            __graphite_経路_by_pair,
            __graphite_construction_stamp,
        })
    }
    /// 最初の1件の違反で `Err` になる版。実装は
    /// `freeze_collecting` に委譲する。
    fn freeze(self) -> Result<Graph, Violation> {
        self.freeze_collecting().map_err(|mut violations| violations.remove(0))
    }
}
/// [`graphite::build_named_graph`] が `#schema_name`/`#violation_ident`
/// の具体型を知らずに凍結を呼べるようにするための橋渡し。
/// `freeze_into_graph` は既存の私有 `freeze()` (上記) へそのまま委譲する。
impl graphite::FreezableBuilder for Builder {
    type Graph = Graph;
    type Violation = Violation;
    fn freeze_into_graph(self) -> Result<Self::Graph, Self::Violation> {
        self.freeze()
    }
}
