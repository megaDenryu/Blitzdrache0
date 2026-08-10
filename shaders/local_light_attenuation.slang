// 局所光の減衰の窓。受け取るのは光からの距離と影響半径、返すのは0以上1以下の係数である。
// 問い合わせの列挙から分けているのは、これがCPU正本
// (`crates/blitz_render/src/clustered_lighting/attenuation_window.rs`)と1対1で対応する算術そのものであり、
// 光の並べ方もクラスタの区間も1つも読まないためである。
//
// 窓を掛けるのは、クラスタの打ち切りと絵を一致させるためである。逆二乗のままでは影響半径の外側でも
// 寄与が0にならず、セルへ割り当てられた光と割り当てられなかった光の境目に段差が出る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 光からの距離が影響半径へ近づくにつれて0へ落ちる、0以上1以下の係数。
float localLightAttenuationWindow(float lightDistance, float influenceRadius)
{
    float ratio = lightDistance / influenceRadius;
    float squaredRatio = ratio * ratio;
    float inner = clamp(1.0 - squaredRatio * squaredRatio, 0.0, 1.0);
    return inner * inner;
}
