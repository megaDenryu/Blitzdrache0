import { DoubleSide, MeshBasicMaterial } from 'three'
import { メッシュ部品, ジオメトリ包み } from 'SengenThree'
import type { 高さ場 } from '../../../編集モデル/index.ts'
import { ブラシ分割数 } from './ブラシ角度列.ts'
import { ブラシ塗り円添字を作る, ブラシ塗り円頂点を更新する } from './ブラシ塗り円頂点計算.ts'

// ブラシ半径の内側を面で見せる半透明の塗り円メッシュ。ブラシリング面部品と同じく
// 頂点ごとに地形の高さへ合わせ、既存バッファへの書き込みで更新する。
export class ブラシ塗り円面部品 extends メッシュ部品<ジオメトリ包み, MeshBasicMaterial> {
    private readonly _頂点バッファ: Float32Array

    public constructor() {
        const 頂点バッファ = new Float32Array((ブラシ分割数 + 1) * 3)
        const ジオメトリ = new ジオメトリ包み()
            .頂点位置を設定する(頂点バッファ)
            .添字を設定する(ブラシ塗り円添字を作る(ブラシ分割数))
        const 材質 = new MeshBasicMaterial({
            color: 0x06b6d4,
            side: DoubleSide,
            transparent: true,
            opacity: 0.15,
            depthTest: false,
        })

        super(ジオメトリ, 材質)
        this._頂点バッファ = 頂点バッファ
        this.実体.visible = false
    }

    public 更新する(角度列: Float64Array, 中心X: number, 中心Z: number, 半径メートル: number, 地形高さ場: 高さ場): void {
        ブラシ塗り円頂点を更新する(this._頂点バッファ, 角度列, 中心X, 中心Z, 半径メートル, 地形高さ場)
        this.ジオメトリ.頂点位置を更新する(this._頂点バッファ)
    }

    public 可視性を設定する(可視: boolean): this {
        this._破棄済みを検査する()
        this.実体.visible = 可視
        return this
    }
}
