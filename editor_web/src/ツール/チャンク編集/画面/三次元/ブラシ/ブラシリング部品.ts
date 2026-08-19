import { DoubleSide, Mesh, RingGeometry, MeshBasicMaterial } from 'three'
import { 三次元部品 } from 'SengenThree'

// 造成および地表ペイント時にポインタ直下の作用半径を地表に示すリング表示部品。
export class ブラシリング部品 extends 三次元部品<Mesh> {
    private readonly _リングメッシュ: Mesh

    public constructor() {
        const 幾何 = new RingGeometry(0.9, 1.0, 32)
        幾何.rotateX(-Math.PI / 2)
        const 材質 = new MeshBasicMaterial({
            color: 0x06b6d4,
            side: DoubleSide,
            transparent: true,
            opacity: 0.8,
            depthTest: false,
        })
        const メッシュ = new Mesh(幾何, 材質)
        メッシュ.visible = false

        super(メッシュ)
        this.資源台帳.登録する(幾何)
        this.資源台帳.登録する(材質)
        this._リングメッシュ = メッシュ
    }

    public 半径を設定する(半径メートル: number): this {
        this._破棄済みを検査する()
        this._リングメッシュ.scale.set(半径メートル, 半径メートル, 1)
        return this
    }

    public 可視性を設定する(可視: boolean): this {
        this._破棄済みを検査する()
        this._リングメッシュ.visible = 可視
        return this
    }

    public 地点へ配置する(x: number, y: number, z: number): this {
        this._破棄済みを検査する()
        this.位置を設定する(x, y + 0.1, z)
        return this
    }
}
