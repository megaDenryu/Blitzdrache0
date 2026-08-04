// 自動露出の導出と時間適応。128個のビンを添字の昇順に単一の呼び出しで走査し、露出状態の永続バッファを更新する。
// 注意: 時間適応の定義の正本は`crates/blitz_engine/src/auto_exposure/adaptation.rs`にあり、ここはその写しである。
// 注意: 呼び出しを1つに保つのは、並列の浮動小数リダクションでは加算順が変わりCPU正本と一致しないためである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import auto_exposure_derivation;
import auto_exposure_state;
import unfused_arithmetic;

[[vk::binding(2, 0)]] StructuredBuffer<uint> histogram;
[[vk::binding(3, 0)]] RWStructuredBuffer<ExposureState> exposureState;

// 導出と適応の狙いと速さ。世界の宣言から起動時に決まるが、即時定数で毎フレーム渡すのは、
// 同じパイプラインを別の設定で走らせる検収の余地を資源の作り直しなしに残すためである。
struct ResolveSetting
{
    float middleGrayLogLuminance;
    float lowerLimit;
    float upperLimit;
    float stepDownSpeed;
    float stepUpSpeed;
    float elapsedSeconds;
};
[[vk::push_constant]] ResolveSetting resolveSetting;

// 1ステップの移動量。速さ×経過時間で頭打ちにし、さらに目標までの残りの差より大きくしない。
// この2つの上限があるかぎり、系列は目標を跨がずに単調に近づく。
float adaptedCompensationStep(float previous, float target)
{
    float difference = unfusedSubtract(target, previous);
    float speed = difference < 0.0 ? resolveSetting.stepDownSpeed : resolveSetting.stepUpSpeed;
    float movementLimit = unfusedMultiply(speed, resolveSetting.elapsedSeconds);
    float movement = unfusedMultiply(min(abs(difference), movementLimit), sign(difference));
    return unfusedAdd(previous, movement);
}

[shader("compute")]
[numthreads(1, 1, 1)]
void computeMain()
{
    DerivationResult derived = deriveTargetCompensationStep(
        histogram,
        resolveSetting.middleGrayLogLuminance,
        resolveSetting.lowerLimit,
        resolveSetting.upperLimit);

    ExposureState state = exposureState[0];
    if (!derived.resolved)
    {
        // 導出不能は正当な入力状態である。追従中なら前の補正段のまま、未初期化なら未初期化のままにする。
        state.unresolvedFrameCount = state.unresolvedFrameCount + 1;
        exposureState[0] = state;
        return;
    }
    state.lastTargetCompensationStep = derived.targetCompensationStep;
    if (state.initialized == 0)
    {
        // 最初の有効なヒストグラムからは補間せずに目標をそのまま初期値にする(起動直後の白飛び・黒潰れを避ける)。
        state.initialized = 1;
        state.compensationStep = derived.targetCompensationStep;
    }
    else
    {
        state.compensationStep = adaptedCompensationStep(state.compensationStep, derived.targetCompensationStep);
    }
    exposureState[0] = state;
}
