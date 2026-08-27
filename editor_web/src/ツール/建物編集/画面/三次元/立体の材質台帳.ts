import type { Material } from 'three'
import { 基本マテリアルを作る, 標準マテリアルを作る } from 'SengenThree'
import { 全ての立体の役割, type 立体の役割 } from '../../編集モデル/index.ts'
import { 役割の識別色を引く } from './役割の識別色.ts'
import { 立体の素材色を引く } from './立体の素材色.ts'

// 役割ごとの材質を、素の姿を塗るものと識別色の重ねを描くものの2組で持つ台帳。
// 見取りを描き直すたびにメッシュは作り直すが、材質は作り直さないため、寿命の違うものを分けて置く。
// 6つの役割ぶんを最初にまとめて作るのは、遅れて作ると生成の通知先を持ち回ることになり、
// 台帳の外へ通知の口を出す形になるためである。
export class 立体の材質台帳 {
    private readonly _素材の材質: Map<立体の役割, Material> = new Map()
    private readonly _識別色の材質: Map<立体の役割, Material> = new Map()

    public constructor() {
        for (const 役割 of 全ての立体の役割) {
            this._素材の材質.set(
                役割,
                標準マテリアルを作る({ 色: 立体の素材色を引く(役割), 粗さ: 0.85, 金属度: 0, 両面描画か: true }),
            )
            this._識別色の材質.set(役割, 基本マテリアルを作る({ 色: 役割の識別色を引く(役割), ワイヤーフレームか: true }))
        }
    }

    public 素材の材質を引く(役割: 立体の役割): Material {
        return this.いずれかの台帳から引く(this._素材の材質, 役割)
    }

    public 識別色の材質を引く(役割: 立体の役割): Material {
        return this.いずれかの台帳から引く(this._識別色の材質, 役割)
    }

    public 全ての材質を数え上げる(): Material[] {
        return [...this._素材の材質.values(), ...this._識別色の材質.values()]
    }

    // 構築時に全ての役割ぶんを入れているため、引けないのは役割の並びと台帳が食い違ったときだけである。
    private いずれかの台帳から引く(台帳: Map<立体の役割, Material>, 役割: 立体の役割): Material {
        const 材質 = 台帳.get(役割)
        if (材質 === undefined) throw new Error(`立体の材質台帳に役割「${役割}」の材質が無い`)
        return 材質
    }
}
