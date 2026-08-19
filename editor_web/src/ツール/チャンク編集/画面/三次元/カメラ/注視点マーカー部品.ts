import { DoubleSide, Group, Mesh, RingGeometry, MeshBasicMaterial } from 'three'
import { 三次元部品 } from 'SengenThree'
import type { 透視カメラ } from 'SengenThree'

// 軌道カメラ制御器の注視点位置を示す二重リングマーカー。ビルボード(常にカメラの正面を
// 向く向き)で表示するため、地形の起伏や視射角に関わらず円形に見える。地形へめり込んでも
// 見えるようdepthTestは無効にしてある。
export class 注視点マーカー部品 extends 三次元部品<Group> {
    private readonly _カメラ: 透視カメラ

    public constructor(カメラ: 透視カメラ) {
        const グループ = new Group()

        const 内側幾何 = new RingGeometry(0.6, 0.9, 32)
        const 内側材質 = 注視点マーカー部品._材質を作る()
        const 内リング = new Mesh(内側幾何, 内側材質)

        const 外側幾何 = new RingGeometry(1.2, 1.4, 32)
        const 外側材質 = 注視点マーカー部品._材質を作る()
        const 外リング = new Mesh(外側幾何, 外側材質)

        グループ.add(内リング, 外リング)
        グループ.visible = false

        super(グループ)
        this.資源台帳.登録する(内側幾何)
        this.資源台帳.登録する(内側材質)
        this.資源台帳.登録する(外側幾何)
        this.資源台帳.登録する(外側材質)
        this._カメラ = カメラ
    }

    private static _材質を作る(): MeshBasicMaterial {
        return new MeshBasicMaterial({
            color: 0xfacc15,
            side: DoubleSide,
            transparent: true,
            opacity: 0.9,
            depthTest: false,
        })
    }

    public 地点へ表示する(x: number, y: number, z: number): this {
        this._破棄済みを検査する()
        this.位置を設定する(x, y, z)
        this.向きを合わせる(this._カメラ)
        this.実体.visible = true
        return this
    }

    public 非表示にする(): this {
        this._破棄済みを検査する()
        this.実体.visible = false
        return this
    }
}
