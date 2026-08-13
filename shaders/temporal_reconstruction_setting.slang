// 時間再構成のパスが押し込む即時定数と、そこから導く2つの規則(深度からビュー空間の奥行き・画素の中心の正規化装置座標)。
// 注意: 並びと値の意味は`crates/blitz_render/src/vulkan/temporal_reconstruction/setting.rs`の写しである。
// 押し込む側の並びが正本であり、片方だけを直すと値が1つずれたまま絵にしか出ない。
// 注意: 奥行きの復元の式は`crates/blitz_render/src/local_visibility/projection.rs`の`奥行きを復元する`と同じである。
// 深度はカメラの逆Z規約であり、1が近クリップ面、0が遠クリップ面である。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct TemporalReconstructionSetting
{
    float nearClip;
    float farClip;
    // 今のフレームの色が結果へ占める割合。正本は`crates/blitz_engine/src/temporal_reconstruction/constants.rs`。
    float currentFrameRatio;
    // 同じ面のものだと認める奥行きの食い違いの上限を、今のフレームの奥行きへの割合で表した値。
    float disocclusionTolerance;
    uint2 extent;
    // 履歴が有効なら1。0のフレームは履歴を1画素も混ぜず、今のフレームの色をそのまま出す。
    // 判断cの「無効化した直後の1フレーム目は今のフレームの結果をそのまま出力する」を画素段で表す値である。
    uint historyValid;
};

// 深度が表すカメラからの奥行き。近クリップ面で近クリップ距離、遠クリップ面で遠クリップ距離になる。
float temporalViewDepthOfDepth(TemporalReconstructionSetting setting, float depth)
{
    return setting.farClip * setting.nearClip / (setting.nearClip + depth * (setting.farClip - setting.nearClip));
}

// 画素の中心が当たる正規化装置座標。横は右が正、縦は下が正である。
// 画素内ずらしを引かないのは、履歴画像の画素が「ずらしを均した後の像」を表すためである。動きベクトルは
// この画素に見えている点が1フレームで画面上を動いた量であり、静止したカメラでは零になる。零のとき履歴を
// 画素の中心そのものから読むことが、標本を積み上げた像を毎フレーム再標本化して鈍らせないための条件である。
float2 temporalPixelCenterNdc(uint2 extent, uint2 pixel)
{
    return float2(
        2.0 * (float(pixel.x) + 0.5) / float(extent.x) - 1.0,
        2.0 * (float(pixel.y) + 0.5) / float(extent.y) - 1.0);
}

// 正規化装置座標をテクスチャの参照位置へ写す。縦はどちらも下向きであるため符号を反転しない。
float2 temporalNdcToUv(float2 ndc)
{
    return ndc * 0.5 + 0.5;
}

// その点が画面の内側にあるか。内側なら真を返す向きで書くのは、非数が偽へ落ちて履歴を捨てる側へ回るためである。
// 「外側なら偽」の向きで書くと、非数の比較がすべて偽になって非数の画素が履歴を採る側へすり抜ける。
bool temporalInsideScreen(float2 ndc)
{
    return ndc.x >= -1.0 && ndc.x <= 1.0 && ndc.y >= -1.0 && ndc.y <= 1.0;
}

// 2つの奥行きが同じ面のものだと認められるか。割合を掛ける相手を今のフレームの側に採るのは、前のフレームの
// 奥行きが写し戻しの誤差を既に含んでおり、基準に据えると許容そのものが揺れるためである。
// これも「認められるなら真」の向きで書く。非数の奥行きは偽へ落ちて出現領域として捨てられる。
bool temporalSameSurface(TemporalReconstructionSetting setting, float previousViewDepth, float currentViewDepth)
{
    float difference = abs(previousViewDepth - currentViewDepth);
    return difference <= setting.disocclusionTolerance * currentViewDepth;
}
