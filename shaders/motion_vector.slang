// 動きベクトルの規約のシェーダー側の写し。担うのは、今のフレームと前のフレームのクリップ位置から正規化装置座標の
// 差分を作ることと、色と動きベクトルの2枚を書く出力の並びを1箇所で宣言することである。
//
// CPU正本は`crates/blitz_engine/src/temporal_reconstruction/motion_vector.rs`である。向きは今のフレームの画素から
// 前のフレームの位置へ向かい、単位は正規化装置座標の差分であり、画素内ずらしの差分は含めない。含めないために、
// 今のフレームの正規化装置座標からビュー定数の画素内ずらしを引いて素の位置へ戻す。前のフレームのビュー射影は
// ずらし無しで渡っているため、そちらからは引かない。
//
// 出力の並びをここに1つだけ置くのは、シーン描画の2つの契約と布の2つの契約と空が同じ2枚の添付へ書くためである。
// 添付の番号と意味の対応が5箇所に散ると、片方だけを直した食い違いが絵にしか出ない。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 参照: `_doc/設計/時間再構成.md`「判断b: 動きベクトルの生成方式」

import view_uniform;

// シーン描画と空のパスが書く2枚の添付。第0が色、第1が動きベクトルである。
struct ColorAndMotionOutput
{
    float4 color : SV_Target0;
    float2 motionVector : SV_Target1;
};

// 今のフレームのクリップ位置と前のフレームのクリップ位置から動きベクトルを作る。
// どちらも透視除算の前の同次座標で受けるのは、画素段への補間が透視補正付きで行われ、除算を画素ごとに行うことで
// はじめてその画素の位置が正確に出るためである。
float2 motionVectorOf(float4 currentClip, float4 previousClip)
{
    float2 currentNdc = currentClip.xy / currentClip.w - viewUniform.pixelJitter.xy;
    float2 previousNdc = previousClip.xy / previousClip.w;
    return previousNdc - currentNdc;
}

// 色と動きベクトルを1つの出力へまとめる。不透明だけを描くため色の第4成分は常に1である。
ColorAndMotionOutput colorAndMotion(float3 color, float4 currentClip, float4 previousClip)
{
    ColorAndMotionOutput output;
    output.color = float4(color, 1.0);
    output.motionVector = motionVectorOf(currentClip, previousClip);
    return output;
}
