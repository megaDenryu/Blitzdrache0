import { グループ } from 'SengenThree'
import type { 高さ場 } from '../../../編集モデル/index.ts'
import { ブラシ角度列を作る, ブラシ分割数, ブラシ内外比 } from './ブラシ角度列.ts'
import { ブラシリング面部品 } from './ブラシリング面部品.ts'
import { ブラシ塗り円面部品 } from './ブラシ塗り円面部品.ts'

// 造成および地表ペイント時にポインタ直下の作用半径を地表に示すブラシ表示。
// リング(輪郭)と塗り円(内側の面)の2つの部品を束ね、中心位置・半径の共有状態を持つ
// (地形へめり込んでも見えるようdepthTestは無効にしてある。参照:
// `_doc/設計/ゲーム開発用エディター基盤.md`判断3)。
export class ブラシリング部品 extends グループ {
    private readonly _リング: ブラシリング面部品
    private readonly _塗り円: ブラシ塗り円面部品
    private readonly _角度列: Float64Array

    private _中心X: number = 0
    private _中心Z: number = 0
    private _半径メートル: number = 1

    public constructor(private readonly _地形高さ場: 高さ場) {
        super()
        this._角度列 = ブラシ角度列を作る(ブラシ分割数)
        this._リング = new ブラシリング面部品()
        this._塗り円 = new ブラシ塗り円面部品()
        this.childs([this._塗り円, this._リング])
    }

    public 半径を設定する(半径メートル: number): this {
        this._破棄済みを検査する()
        this._半径メートル = 半径メートル
        this._頂点を再計算する()
        return this
    }

    public 可視性を設定する(可視: boolean): this {
        this._破棄済みを検査する()
        this._リング.可視性を設定する(可視)
        this._塗り円.可視性を設定する(可視)
        return this
    }

    // yは呼び出し元(レイキャストの交差点)から渡るが、実際のY座標は地形の高さ場から
    // 頂点ごとに再サンプリングするため使わない(起伏に沿わせるにはオブジェクト単位の
    // Y位置ではなく頂点ごとの高さが要る)。
    public 地点へ配置する(x: number, _y: number, z: number): this {
        this._破棄済みを検査する()
        this._中心X = x
        this._中心Z = z
        this.位置を設定する(x, 0, z)
        this._頂点を再計算する()
        return this
    }

    private _頂点を再計算する(): void {
        this._リング.更新する(this._角度列, this._中心X, this._中心Z, this._半径メートル, this._地形高さ場)
        this._塗り円.更新する(this._角度列, this._中心X, this._中心Z, this._半径メートル * ブラシ内外比, this._地形高さ場)
    }
}
