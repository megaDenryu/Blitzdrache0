import { Mesh, SphereGeometry, MeshStandardMaterial } from 'three'
import { グループ, ジオメトリ包み } from 'SengenThree'
import type { 道路スプライン } from '../../../編集モデル/index.ts'

// 道路制御点の位置に球体マーカーを表示し、選択状態を描き分ける部品。
export class 道路ノードメッシュ部品 extends グループ {
    private readonly _球ジオメトリ: ジオメトリ包み
    private readonly _通常材質: MeshStandardMaterial
    private readonly _選択材質: MeshStandardMaterial
    private _ノードメッシュ一覧: Mesh[] = []

    public constructor() {
        super()
        this._球ジオメトリ = new ジオメトリ包み(new SphereGeometry(0.8, 16, 16))
        this._通常材質 = new MeshStandardMaterial({ color: 0x06b6d4, emissive: 0x083344 })
        this._選択材質 = new MeshStandardMaterial({ color: 0xf59e0b, emissive: 0x451a03 })

        this.資源台帳.登録する(this._球ジオメトリ)
        this.資源台帳.登録する(this._通常材質)
        this.資源台帳.登録する(this._選択材質)
    }

    public get ノードメッシュ一覧(): readonly Mesh[] {
        return this._ノードメッシュ一覧
    }

    public 更新する(スプライン: 道路スプライン, 選択中添字: number): void {
        for (const m of this._ノードメッシュ一覧) {
            this.実体.remove(m)
        }
        this._ノードメッシュ一覧 = []

        for (let i = 0; i < スプライン.制御点列.length; i++) {
            const 点 = スプライン.制御点列[i]
            if (点 === undefined) continue
            const 材質 = i === 選択中添字 ? this._選択材質 : this._通常材質
            const メッシュ = new Mesh(this._球ジオメトリ.実体, 材質)
            メッシュ.position.set(点.x, 点.y, 点.z)
            this.実体.add(メッシュ)
            this._ノードメッシュ一覧.push(メッシュ)
        }
    }
}
