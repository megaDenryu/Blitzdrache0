import { ShaderMaterial, Color, DataTexture } from 'three'

// チャンク編集の4層スプラットマップ（草・泥・岩・砂）ブレンドと、大域編集向けの
// 標高グラデーション表示を、1つのシェーダーの分岐で切り替える地形マテリアル。
// 標高グラデーションは判断9(識別色で足りる)の範囲の表示専用の色分けであり、
// 地表材質(スプラット)の値を書き換えない。参照: `_doc/設計/ゲーム開発用エディター基盤.md`。
// 注意: GLSL文字列の内側(uniform名・関数名・ローカル変数名)はGLSLがASCII識別子しか
// 受け付けないため英語のままにする。TS側の公開API(型名・関数名・引数名)は日本語で書く。

// 標高グラデーションの3段の色。低い側から中間・高い側へ線形に混ぜる。
export interface 標高グラデーション配色 {
    readonly 低色: number
    readonly 中色: number
    readonly 高色: number
}

const 頂点シェーダー = `
varying vec2 vUv;
varying vec3 vNormal;
varying float vHeight;

void main() {
    vUv = uv;
    vNormal = normalize(normalMatrix * normal);
    vHeight = position.y;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
`

const フラグメントシェーダー = `
uniform sampler2D splatTex;
uniform vec3 colGrass;
uniform vec3 colDirt;
uniform vec3 colRock;
uniform vec3 colSand;
uniform float elevGradEnabled;
uniform vec3 elevColorLow;
uniform vec3 elevColorMid;
uniform vec3 elevColorHigh;
uniform float elevHeightMin;
uniform float elevHeightMax;
varying vec2 vUv;
varying vec3 vNormal;
varying float vHeight;

// 注意: GLSLの識別子はASCIIのみ受け付ける(非ASCII識別子は構文エラーになる)。
// このシェーダー文字列内だけは英語命名の例外とし、TS側の公開APIは引き続き日本語で書く。
vec3 elevationGradientColor(float height) {
    float span = max(elevHeightMax - elevHeightMin, 0.001);
    float t = clamp((height - elevHeightMin) / span, 0.0, 1.0);
    if (t < 0.5) {
        return mix(elevColorLow, elevColorMid, t * 2.0);
    }
    return mix(elevColorMid, elevColorHigh, (t - 0.5) * 2.0);
}

void main() {
    vec3 baseColor;
    if (elevGradEnabled > 0.5) {
        baseColor = elevationGradientColor(vHeight);
    } else {
        vec4 splat = texture2D(splatTex, vUv);
        float total = splat.r + splat.g + splat.b + splat.a;
        if (total > 0.0) {
            splat /= total;
        } else {
            splat = vec4(1.0, 0.0, 0.0, 0.0);
        }
        baseColor = colGrass * splat.r + colDirt * splat.g + colRock * splat.b + colSand * splat.a;
    }
    // 光源の仰角を約54度(旧: normalize(0.5,0.8,0.3))から約38度へ下げて斜めからの
    // 陰影(レイキング光)にし、緩やかな起伏でも法線差が明暗として見えるようにする。
    // あわせて環境光の下限を0.35から0.3へ、拡散反射の比重を0.7から0.75へ上げて
    // コントラストを増した。識別色の判別を妨げないため上限(diff=1.0時の1.05)は
    // ほぼ据え置く。
    vec3 lightDir = normalize(vec3(0.6, 0.6, 0.35));
    float diff = max(dot(vNormal, lightDir), 0.0);
    vec3 finalColor = baseColor * (diff * 0.75 + 0.3);
    gl_FragColor = vec4(finalColor, 1.0);
}
`

// 標高グラデーション配色を渡すとその表示モードで、渡さないとスプラットブレンド表示で
// マテリアルを生成する。有効・無効はelevGradEnabled uniformの分岐で切り替わる。
export function 地形マテリアルを生成する(
    スプラットテクスチャ: DataTexture,
    標高グラデーション?: 標高グラデーション配色,
): ShaderMaterial {
    return new ShaderMaterial({
        uniforms: {
            splatTex: { value: スプラットテクスチャ },
            colGrass: { value: new Color(0x2d5a27) },
            colDirt: { value: new Color(0x5c4033) },
            colRock: { value: new Color(0x64748b) },
            colSand: { value: new Color(0xd4b483) },
            elevGradEnabled: { value: 標高グラデーション !== undefined ? 1 : 0 },
            elevColorLow: { value: new Color(標高グラデーション?.低色 ?? 0x000000) },
            elevColorMid: { value: new Color(標高グラデーション?.中色 ?? 0x000000) },
            elevColorHigh: { value: new Color(標高グラデーション?.高色 ?? 0x000000) },
            elevHeightMin: { value: 0 },
            elevHeightMax: { value: 1 },
        },
        vertexShader: 頂点シェーダー,
        fragmentShader: フラグメントシェーダー,
    })
}
