import { ShaderMaterial, Color, DataTexture } from 'three'

// 4層スプラットマップ（草・泥・岩・砂）の重みに基づいてマルチテクスチャブレンドを行うシェーダーマテリアル。
const 頂点シェーダー = `
varying vec2 vUv;
varying vec3 vNormal;

void main() {
    vUv = uv;
    vNormal = normalize(normalMatrix * normal);
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
`

const フラグメントシェーダー = `
uniform sampler2D splatTex;
uniform vec3 colGrass;
uniform vec3 colDirt;
uniform vec3 colRock;
uniform vec3 colSand;
varying vec2 vUv;
varying vec3 vNormal;

void main() {
    vec4 splat = texture2D(splatTex, vUv);
    float total = splat.r + splat.g + splat.b + splat.a;
    if (total > 0.0) {
        splat /= total;
    } else {
        splat = vec4(1.0, 0.0, 0.0, 0.0);
    }
    vec3 baseColor = colGrass * splat.r + colDirt * splat.g + colRock * splat.b + colSand * splat.a;
    vec3 lightDir = normalize(vec3(0.5, 0.8, 0.3));
    float diff = max(dot(vNormal, lightDir), 0.0);
    vec3 finalColor = baseColor * (diff * 0.7 + 0.35);
    gl_FragColor = vec4(finalColor, 1.0);
}
`

export function 地形マテリアルを生成する(スプラットテクスチャ: DataTexture): ShaderMaterial {
    return new ShaderMaterial({
        uniforms: {
            splatTex: { value: スプラットテクスチャ },
            colGrass: { value: new Color(0x2d5a27) },
            colDirt: { value: new Color(0x5c4033) },
            colRock: { value: new Color(0x64748b) },
            colSand: { value: new Color(0xd4b483) },
        },
        vertexShader: 頂点シェーダー,
        fragmentShader: フラグメントシェーダー,
    })
}
