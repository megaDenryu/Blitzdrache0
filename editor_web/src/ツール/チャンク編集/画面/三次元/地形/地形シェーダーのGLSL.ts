// 地形マテリアルのGLSLの本文。頂点シェーダーとフラグメントシェーダーの2本の綴りを所有し、
// マテリアルの組み立て(地形シェーダー.ts)からは文字列として受け取るだけにする。
// 注意: GLSL文字列の内側(uniform名・関数名・ローカル変数名)はGLSLがASCII識別子しか
// 受け付けないため英語のままにする。TS側の公開API(型名・関数名・引数名)は日本語で書く。

export const 頂点シェーダー = `
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

export const フラグメントシェーダー = `
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
