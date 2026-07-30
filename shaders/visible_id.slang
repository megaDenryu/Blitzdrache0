// そのフレームに描く個体の添字の列(binding7)。LOD段ごとの連続した範囲に並び、各段の範囲の先頭側に
// シーンパスが描く可視個体が置かれる。シーンとシャドウのどちらの頂点シェーダーも、SV_StartInstanceLocationで
// 受け取る段の範囲の開始へSV_InstanceIDを足してこの列を参照し、得た添字で個体変換(binding6)を参照する。
// 可視判定とLODの選択で描く個体が変わっても個体変換は書き直さず、この列だけが毎フレーム変わる。
// シーンパスは段の範囲の先頭から可視数ぶんを、シャドウパスは同じ範囲の全体を描く。
// 注意: シャドウパスのインスタンス数は常にその段の全個体である。カメラ視錐台で絞った可視集合をシャドウへ
// 流用してはならない(参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」)。
// 個体が1体だけの対象は、束が共有する値0だけの列を指す。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

[[vk::binding(7, 0)]]
StructuredBuffer<uint> visibleInstanceIds;

// 段の範囲の開始とdraw内のインスタンス番号から、この頂点が描く個体の添字を参照する。
uint resolveInstanceIndex(uint instanceIndex, uint startInstance)
{
    return visibleInstanceIds[instanceIndex + startInstance];
}
