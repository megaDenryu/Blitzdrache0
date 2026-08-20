import { ShaderMaterial, Color, DataTexture } from 'three'
import { フラグメントシェーダー, 頂点シェーダー } from './地形シェーダーのGLSL.ts'

// チャンク編集の4層スプラットマップ（草・泥・岩・砂）ブレンドと、大域編集向けの
// 標高グラデーション表示を、1つのシェーダーの分岐で切り替える地形マテリアル。
// 標高グラデーションは判断9(識別色で足りる)の範囲の表示専用の色分けであり、
// 地表材質(スプラット)の値を書き換えない。参照: `_doc/設計/ゲーム開発用エディター基盤.md`。

// 標高グラデーションの3段の色。低い側から中間・高い側へ線形に混ぜる。
export interface 標高グラデーション配色 {
    readonly 低色: number
    readonly 中色: number
    readonly 高色: number
}

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
