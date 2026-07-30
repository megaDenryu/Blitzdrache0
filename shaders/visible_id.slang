// そのフレームに描く個体の添字の列(binding7)。パスの順(シーン→距離区分0から距離区分3)に区間が並び、各パスの区間の内側が
// LOD段の順に並ぶ。シーンとシャドウのどちらの頂点シェーダーも、SV_StartInstanceLocationで受け取る自分の区間の開始へ
// SV_InstanceIDを足してこの列を参照し、得た添字で個体変換(binding6)を参照する。
// 可視判定とLODの選択で描く個体が変わっても個体変換は書き直さず、この列だけが毎フレーム変わる。
// 同じ個体の添字が複数の距離区分の区間へ現れることを許すため、列は全個体の並べ替えではなく、長さは毎フレーム変わる。
// 注意: 距離区分ごとのインスタンス数はその距離区分のライトビュー射影から導いた視錐台が通した数である。カメラ視錐台で絞った
// 可視集合を影へ流用してはならない
// (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「可視ID列の距離区分別の区間(4区分並べの後継)」)。
// 個体が1体だけの対象は、束が共有する値0だけの列を指す。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

[[vk::binding(7, 0)]]
StructuredBuffer<uint> visibleInstanceIds;

// 自分の区間の開始とdraw内のインスタンス番号から、この頂点が描く個体の添字を参照する。
uint resolveInstanceIndex(uint instanceIndex, uint startInstance)
{
    return visibleInstanceIds[instanceIndex + startInstance];
}
