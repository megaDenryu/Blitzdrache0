import { DataTexture, RGBAFormat, UnsignedByteType, LinearFilter, ClampToEdgeWrapping, Color, ShaderMaterial } from 'three'
import { メッシュ部品, ジオメトリ包み } from 'SengenThree'
import type { 高さ場, 地表材質 } from '../../../編集モデル/index.ts'
import { 地形幾何データを生成する, 地形頂点高さを更新する, 高さの範囲を求める } from './地形頂点計算.ts'
import { 地形マテリアルを生成する, type 標高グラデーション配色 } from './地形シェーダー.ts'

// 高さ場と地表材質データを保持し、ジオメトリバッファとテクスチャの高速更新を提供する地形メッシュ部品。
// 標高グラデーションを渡すと大域編集向けの標高色分け表示になり、以降の高さ場更新のたびに
// 標高範囲を再計算する(渡さないチャンク編集側は従来どおりスプラットブレンド表示のまま)。
export class 地形メッシュ部品 extends メッシュ部品<ジオメトリ包み, ShaderMaterial> {
    private readonly _解像度: number
    private readonly _スプラットテクスチャ: DataTexture
    private readonly _頂点バッファ: Float32Array
    private readonly _標高グラデーション有効: boolean

    public constructor(
        高さ場モデル: 高さ場,
        地表材質モデル: 地表材質,
        標高グラデーション?: 標高グラデーション配色,
    ) {
        const 解像度 = 高さ場モデル.解像度
        const 一辺 = 高さ場モデル.一辺のメートル
        const { 頂点配列, 法線配列, UV配列, 添字配列 } = 地形幾何データを生成する(
            解像度,
            一辺,
            高さ場モデル.格子データ,
        )

        const ジオメトリ = new ジオメトリ包み()
            .頂点位置を設定する(頂点配列)
            .法線ベクトルを設定する(法線配列)
            .UV座標を設定する(UV配列)
            .添字を設定する(添字配列)
            .法線を自動計算する()

        const スプラットテクスチャ = new DataTexture(
            地表材質モデル.材質データ,
            解像度,
            解像度,
            RGBAFormat,
            UnsignedByteType,
        )
        スプラットテクスチャ.magFilter = LinearFilter
        スプラットテクスチャ.minFilter = LinearFilter
        スプラットテクスチャ.wrapS = ClampToEdgeWrapping
        スプラットテクスチャ.wrapT = ClampToEdgeWrapping
        スプラットテクスチャ.needsUpdate = true

        const マテリアル = 地形マテリアルを生成する(スプラットテクスチャ, 標高グラデーション)

        super(ジオメトリ, マテリアル)
        this.資源台帳.登録する(スプラットテクスチャ)
        this._解像度 = 解像度
        this._スプラットテクスチャ = スプラットテクスチャ
        this._頂点バッファ = 頂点配列
        this._標高グラデーション有効 = 標高グラデーション !== undefined
        if (this._標高グラデーション有効) {
            this.標高範囲を更新する(高さ場モデル.格子データ)
        }
    }

    public 高さ場を更新する(高さ場モデル: 高さ場): void {
        地形頂点高さを更新する(this._頂点バッファ, this._解像度, 高さ場モデル.格子データ)
        this.ジオメトリ.頂点位置を更新する(this._頂点バッファ)
        this.ジオメトリ.法線を自動計算する()
        if (this._標高グラデーション有効) {
            this.標高範囲を更新する(高さ場モデル.格子データ)
        }
    }

    public 地表材質を更新する(): void {
        this._スプラットテクスチャ.needsUpdate = true
    }

    // テーマ切替時に地形の基本色(草の色)を差し替える。他の3層(泥・岩・砂)は
    // テーマに依存しない地質色として据え置く(参照: 工房テーマ/夜間テーマの地形基本色)。
    public 地形色を更新する(地形基本色: number): void {
        this.マテリアル.uniforms.colGrass.value = new Color(地形基本色)
    }

    // テーマ切替時に大域編集の標高グラデーション3色を差し替える。
    public 標高配色を更新する(配色: 標高グラデーション配色): void {
        this.マテリアル.uniforms.elevColorLow.value = new Color(配色.低色)
        this.マテリアル.uniforms.elevColorMid.value = new Color(配色.中色)
        this.マテリアル.uniforms.elevColorHigh.value = new Color(配色.高色)
    }

    private 標高範囲を更新する(高さデータ: Float32Array): void {
        const 範囲 = 高さの範囲を求める(高さデータ)
        this.マテリアル.uniforms.elevHeightMin.value = 範囲.最小
        this.マテリアル.uniforms.elevHeightMax.value = 範囲.最大
    }
}
