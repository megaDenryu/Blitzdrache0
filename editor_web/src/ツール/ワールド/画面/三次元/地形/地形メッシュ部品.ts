import { DataTexture, RGBAFormat, UnsignedByteType, LinearFilter, ClampToEdgeWrapping } from 'three'
import { メッシュ部品, ジオメトリ包み } from 'SengenThree'
import type { 高さ場, 地表材質 } from '../../../編集モデル/index.ts'
import { 地形幾何データを生成する, 地形頂点高さを更新する } from './地形頂点計算.ts'
import { 地形マテリアルを生成する } from './地形シェーダー.ts'

// 高さ場と地表材質データを保持し、ジオメトリバッファとテクスチャの高速更新を提供する地形メッシュ部品。
export class 地形メッシュ部品 extends メッシュ部品<ジオメトリ包み> {
    private readonly _解像度: number
    private readonly _スプラットテクスチャ: DataTexture
    private readonly _頂点バッファ: Float32Array

    public constructor(
        高さ場モデル: 高さ場,
        地表材質モデル: 地表材質,
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

        const マテリアル = 地形マテリアルを生成する(スプラットテクスチャ)

        super(ジオメトリ, マテリアル)
        this.資源台帳.登録する(スプラットテクスチャ)
        this._解像度 = 解像度
        this._スプラットテクスチャ = スプラットテクスチャ
        this._頂点バッファ = 頂点配列
    }

    public 高さ場を更新する(高さ場モデル: 高さ場): void {
        地形頂点高さを更新する(this._頂点バッファ, this._解像度, 高さ場モデル.格子データ)
        this.ジオメトリ.頂点位置を更新する(this._頂点バッファ)
        this.ジオメトリ.法線を自動計算する()
    }

    public 地表材質を更新する(): void {
        this._スプラットテクスチャ.needsUpdate = true
    }
}
