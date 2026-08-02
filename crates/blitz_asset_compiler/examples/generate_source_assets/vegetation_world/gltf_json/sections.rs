//! メッシュLOD段1つぶんのglTF宣言を作る工程。受け取るのは段番号と直方体の諸元と段の中身の量、
//! 返すのはその段のbufferView5件とaccessor5件とmesh1件の宣言文字列である。
//! 段ごとにbufferViewとaccessorが5件ずつ増え、番号は段番号の5倍から始まる。
//! この対応をここ1箇所で決めることで、文書の骨組み側は段の中身の並びを知らずに済む。

use super::super::geometry::直方体諸元;
use super::super::stage_amount::段の中身の量;

/// 1つの段が使うbufferViewとaccessorの件数。位置・法線・接線・UV・インデックスの5つである。
const 段あたりの区間数: usize = 5;

pub(super) struct 段の区間 {
    pub(super) bufferview宣言: String,
    pub(super) accessor宣言: String,
}

pub(super) fn 段の区間を作る(段番号: usize, 諸元: 直方体諸元, 量: 段の中身の量) -> 段の区間 {
    let (位置区間長, 法線区間長, 接線区間長) = (量.位置区間長(), 量.法線区間長(), 量.接線区間長());
    let (テクスチャ座標区間長, インデックス区間長) = (量.テクスチャ座標区間長(), 量.インデックス区間長());
    let (頂点数, インデックス数) = (量.頂点数, 量.インデックス数);
    let 基点 = 量.バイト長() * 段番号;
    let 法線区間 = 基点 + 位置区間長;
    let 接線区間 = 法線区間 + 法線区間長;
    let uv区間 = 接線区間 + 接線区間長;
    let インデックス区間 = uv区間 + テクスチャ座標区間長;
    let 最小 = format!("[{}, 0.0, {}]", -諸元.半辺, -諸元.半辺);
    let 最大 = format!("[{}, {}, {}]", 諸元.半辺, 諸元.高さ, 諸元.半辺);
    let 番号 = 段番号 * 段あたりの区間数;
    段の区間 {
        bufferview宣言: format!(
            "    {{ \"buffer\": 0, \"byteOffset\": {基点}, \"byteLength\": {位置区間長}, \"target\": 34962 }},\n\
             \x20   {{ \"buffer\": 0, \"byteOffset\": {法線区間}, \"byteLength\": {法線区間長}, \"target\": 34962 }},\n\
             \x20   {{ \"buffer\": 0, \"byteOffset\": {接線区間}, \"byteLength\": {接線区間長}, \"target\": 34962 }},\n\
             \x20   {{ \"buffer\": 0, \"byteOffset\": {uv区間}, \"byteLength\": {テクスチャ座標区間長}, \"target\": 34962 }},\n\
             \x20   {{ \"buffer\": 0, \"byteOffset\": {インデックス区間}, \"byteLength\": {インデックス区間長}, \"target\": 34963 }}"
        ),
        accessor宣言: format!(
            "    {{ \"bufferView\": {}, \"componentType\": 5126, \"count\": {頂点数}, \"type\": \"VEC3\", \"min\": {最小}, \"max\": {最大} }},\n\
             \x20   {{ \"bufferView\": {}, \"componentType\": 5126, \"count\": {頂点数}, \"type\": \"VEC3\" }},\n\
             \x20   {{ \"bufferView\": {}, \"componentType\": 5126, \"count\": {頂点数}, \"type\": \"VEC4\" }},\n\
             \x20   {{ \"bufferView\": {}, \"componentType\": 5126, \"count\": {頂点数}, \"type\": \"VEC2\" }},\n\
             \x20   {{ \"bufferView\": {}, \"componentType\": 5123, \"count\": {インデックス数}, \"type\": \"SCALAR\" }}",
            番号,
            番号 + 1,
            番号 + 2,
            番号 + 3,
            番号 + 4
        ),
    }
}

pub(super) fn mesh宣言を作る(段番号: usize) -> String {
    let 番号 = 段番号 * 段あたりの区間数;
    format!(
        "    {{\n\
         \x20     \"primitives\": [\n\
         \x20       {{\n\
         \x20         \"attributes\": {{ \"POSITION\": {}, \"NORMAL\": {}, \"TANGENT\": {}, \"TEXCOORD_0\": {} }},\n\
         \x20         \"indices\": {}\n\
         \x20       }}\n\
         \x20     ]\n\
         \x20   }}",
        番号,
        番号 + 1,
        番号 + 2,
        番号 + 3,
        番号 + 4
    )
}
