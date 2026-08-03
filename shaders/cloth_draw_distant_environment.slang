// 遠方環境の契約の布の描画の画素段。定数近似の契約との違いは間接光の項の出どころだけであり、
// 法線・織り目・方向光の合計は`cloth_shading.slang`を`cloth_draw.slang`と共有する。
// 頂点段は`cloth_draw.slang`のものをそのまま使うため、このファイルは画素段のエントリだけを持つ。
// 布は金属でないため取り分を掛けず、拡散照度をLambertの規約で割った値をそのまま間接光にする
// (参照: `_doc/設計/放射輝度問い合わせ階層.md`「世界の間接照明方針と契約の2枝(3-Ic)」)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cloth_io;
import cloth_shading;
import indirect_distant_environment;
import lighting_query;

[shader("fragment")]
float4 fragmentMain(VertexOutput input, bool isFrontFace: SV_IsFrontFace) : SV_Target
{
    float3 normal = clothSurfaceNormal(input.normal, isFrontFace);
    float3 direct = clothDirectLight(normal, input.relativePosition);
    float3 ambient = distantEnvironmentClothIndirect(normal, lightingQueryHeader.sunDirection.xyz);
    return float4(clothColor * clothWeave(input.uv) * (direct + ambient), 1.0);
}
