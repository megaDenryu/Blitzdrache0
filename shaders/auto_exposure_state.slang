// 世界の自動露出の状態。1本の永続バッファに1件だけ置き、前フレームが書いた値を次フレームが読む。
// 注意: 対応するCPU側の並びは`crates/blitz_render/src/vulkan/auto_exposure/storage.rs`にあり、
// 初期化(全要素0=未初期化)もそこが行う。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct ExposureState
{
    // 有効なヒストグラムから1度でも補正段を作ったかどうか。0のあいだ`compensationStep`は読んではならない。
    uint initialized;
    // 追従中の露出補正段(EV)。
    float compensationStep;
    // ビンへ入った画素が1つも無く導出できなかったフレームの累計。状態を変えなかった回数がそのまま増える。
    uint unresolvedFrameCount;
    // 直近に導出できたフレームの目標補正段(EV)。時間適応を通す前の値であり、CPU正本との一致を検収が読む。
    float lastTargetCompensationStep;
};

// 描くフレームが使う自動補正段。未初期化のあいだは0であり、時刻別固定へは戻さない。
// 注意: 状態そのものを0で初期化するのではなく、描画値へ変換するこの境界だけが0を使う。
float automaticCompensationStep(ExposureState state)
{
    if (state.initialized == 0)
    {
        return 0.0;
    }
    return state.compensationStep;
}
