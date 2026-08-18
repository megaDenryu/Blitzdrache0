import { Vector3, Quaternion, Matrix4, ConeGeometry, MeshStandardMaterial } from 'three'
import { 個体群部品, ジオメトリ包み } from 'SengenThree'
import type { 散布点, 高さ場 } from '../../../編集モデル/index.ts'
import { 散布 } from '../../../編集モデル/index.ts'

// 植生等の大量インスタンスを単一ドローコールで高速描画・更新する散布個体群部品。
export class 散布個体群部品 extends 個体群部品<ジオメトリ包み, MeshStandardMaterial> {
    private readonly _作業用位置: Vector3 = new Vector3()
    private readonly _作業用回転: Quaternion = new Quaternion()
    private readonly _作業用拡大: Vector3 = new Vector3()
    private readonly _作業用行列: Matrix4 = new Matrix4()
    private readonly _上方向: Vector3 = new Vector3(0, 1, 0)
    private readonly _法線作業: Vector3 = new Vector3()

    public constructor() {
        const 幾何 = new ConeGeometry(1.2, 4.5, 5)
        幾何.translate(0, 2.25, 0)
        const 包み幾何 = new ジオメトリ包み(幾何)
        const 材質 = new MeshStandardMaterial({ color: 0x10b981, roughness: 0.9 })

        super(包み幾何, 材質, 散布.最大散布数)
    }

    public 散布点を更新する(散布点一覧: ReadonlyArray<散布点>, 高さ場モデル: 高さ場): void {
        const 表示個数 = Math.min(散布点一覧.length, this.容量)

        for (let i = 0; i < 表示個数; i++) {
            const 点 = 散布点一覧[i]
            if (点 === undefined) continue
            const y = 高さ場モデル.標本高さを取得する(点.x, 点.z)
            const 法線 = 高さ場モデル.標本法線を取得する(点.x, 点.z)

            this._法線作業.set(法線.x, 法線.y, 法線.z).normalize()
            this._作業用位置.set(点.x, y, 点.z)
            this._作業用回転.setFromUnitVectors(this._上方向, this._法線作業)
            const スケール = 0.8 + (Math.sin(点.x * 5.0) * 0.5 + 0.5) * 0.5
            this._作業用拡大.set(スケール, スケール, スケール)

            this._作業用行列.compose(this._作業用位置, this._作業用回転, this._作業用拡大)
            this.個体の行列を設定する(i, this._作業用行列)
        }

        this.表示個体数を設定する(表示個数)
        this.行列を一括更新する()
    }
}
