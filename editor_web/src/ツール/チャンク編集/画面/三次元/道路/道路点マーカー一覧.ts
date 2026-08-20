import { Group, Mesh, Object3D, Vector3 } from 'three'
import type { BufferGeometry } from 'three'
import type { 道路の一覧, 道路点の在り処 } from '../../../編集モデル/index.ts'
import type { 道路点マーカー材質束 } from './道路点マーカー材質束.ts'
import { 道路点の見た目の状態を決める, type 道路点マーカーの表示の指定 } from './道路点マーカーの見た目の状態.ts'

// マーカーは地表に沿った円盤として置き、地面と同一平面で描かれてちらつかないようこの高さだけ浮かせる。
const 地表から浮かせるメートル = 0.25
// カメラからの距離に掛けて世界での大きさを決める比率。画角50度の画面でおよそ一定の見かけ大きさになる。
const カメラ距離に対する見かけの大きさ = 0.014
const 世界での最小の大きさ = 0.5
const 世界での最大の大きさ = 40

interface 置かれたマーカー {
    readonly 部品: Group
    readonly 在り処: 道路点の在り処
}

// 道路一覧の全ての制御点ぶんのマーカーの実体を所有し、置き直し・当たり判定・見かけの大きさの
// 追従を担う。形状と材質の資源は道路点マーカー部品が所有し、ここは借りて使う。
export class 道路点マーカー一覧 {
    private _置かれた一覧: 置かれたマーカー[] = []

    public constructor(
        private readonly _親: Object3D,
        private readonly _輪の形状: BufferGeometry,
        private readonly _中心の形状: BufferGeometry,
        private readonly _材質束: 道路点マーカー材質束,
    ) {}

    public 置き直す(道路一覧: 道路の一覧, 指定: 道路点マーカーの表示の指定): void {
        for (const 置かれた of this._置かれた一覧) this._親.remove(置かれた.部品)
        this._置かれた一覧 = []

        道路一覧.全ての道路.forEach((道路, 道路添字) => {
            道路.制御点列.forEach((点, 制御点添字) => {
                const 在り処: 道路点の在り処 = { 道路添字, 制御点添字 }
                const 部品 = this._一点ぶんのマーカーを作る(道路点の見た目の状態を決める(在り処, 指定))
                部品.position.set(点.x, 点.y + 地表から浮かせるメートル, 点.z)
                this._親.add(部品)
                this._置かれた一覧.push({ 部品, 在り処 })
            })
        })
    }

    public 当たった道路点の在り処を求める(交差した物体: Object3D): 道路点の在り処 | null {
        for (const 置かれた of this._置かれた一覧) {
            let 対象: Object3D | null = 交差した物体
            while (対象 !== null) {
                if (対象 === 置かれた.部品) return 置かれた.在り処
                対象 = 対象.parent
            }
        }
        return null
    }

    // カメラから遠い点ほど世界での大きさを増やし、画面上の見かけの大きさをそろえる。
    public 見かけの大きさを合わせる(カメラ位置: Vector3): void {
        for (const 置かれた of this._置かれた一覧) {
            const 距離 = 置かれた.部品.position.distanceTo(カメラ位置)
            const 大きさ = Math.min(
                Math.max(距離 * カメラ距離に対する見かけの大きさ, 世界での最小の大きさ),
                世界での最大の大きさ,
            )
            置かれた.部品.scale.setScalar(大きさ)
        }
    }

    private _一点ぶんのマーカーを作る(状態: Parameters<道路点マーカー材質束['状態の材質を取り出す']>[0]): Group {
        const 材質 = this._材質束.状態の材質を取り出す(状態)
        const マーカー = new Group()
        // 奥行き判定を切った物体は描画順で前後が決まるため、地形より後に描かれるよう並びを最後尾へ寄せる。
        const 輪 = new Mesh(this._輪の形状, 材質)
        const 中心 = new Mesh(this._中心の形状, 材質)
        輪.renderOrder = 10
        中心.renderOrder = 10
        マーカー.add(輪, 中心)
        return マーカー
    }
}
