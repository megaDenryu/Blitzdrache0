import { CircleGeometry, Group, Mesh, Object3D, RingGeometry, Vector3 } from 'three'
import { グループ, ジオメトリ包み } from 'SengenThree'
import type { 透視カメラ } from 'SengenThree'
import type { 道路スプライン } from '../../../編集モデル/index.ts'
import { 道路点マーカー材質束, type 道路点マーカー配色, type 道路点マーカーの状態 } from './道路点マーカー材質束.ts'

// マーカーは地表に沿った円盤として置き、地面と同一平面で描かれてちらつかないようこの高さだけ浮かせる。
const 地表から浮かせるメートル = 0.25
// カメラからの距離に掛けて世界での大きさを決める比率。画角50度の画面でおよそ一定の見かけ大きさになる。
const カメラ距離に対する見かけの大きさ = 0.014
const 世界での最小の大きさ = 0.5
const 世界での最大の大きさ = 40

// 道路の制御点の位置に、地表へ伏せた輪と中心点のマーカーを並べる部品。
// 見かけの大きさをカメラ距離へ追従させるため、毎フレーム`見かけの大きさを更新する`を呼ぶ。
export class 道路点マーカー部品 extends グループ {
    private readonly _輪ジオメトリ: ジオメトリ包み
    private readonly _中心ジオメトリ: ジオメトリ包み
    private readonly _材質束: 道路点マーカー材質束
    private readonly _カメラ位置の作業用ベクトル = new Vector3()
    private _マーカー一覧: Group[] = []

    public constructor(
        private readonly _カメラ: 透視カメラ,
        初期配色: 道路点マーカー配色,
    ) {
        super()
        this._輪ジオメトリ = new ジオメトリ包み(new RingGeometry(0.72, 1.0, 28))
        this._中心ジオメトリ = new ジオメトリ包み(new CircleGeometry(0.34, 20))
        this._輪ジオメトリ.実体.rotateX(-Math.PI / 2)
        this._中心ジオメトリ.実体.rotateX(-Math.PI / 2)
        this._材質束 = new 道路点マーカー材質束(初期配色)

        this.資源台帳.登録する(this._輪ジオメトリ)
        this.資源台帳.登録する(this._中心ジオメトリ)
        for (const 材質 of this._材質束.材質一覧) this.資源台帳.登録する(材質)
    }

    // 交差した物体からその物体が属する道路点の添字を求める。当たっていなければnullを返す。
    public 当たった道路点の添字を求める(交差した物体: Object3D): number | null {
        for (let 添字 = 0; 添字 < this._マーカー一覧.length; 添字++) {
            const マーカー = this._マーカー一覧[添字]
            if (マーカー === undefined) continue
            let 対象: Object3D | null = 交差した物体
            while (対象 !== null) {
                if (対象 === マーカー) return 添字
                対象 = 対象.parent
            }
        }
        return null
    }

    public 表示するか設定する(表示するか: boolean): void {
        this.実体.visible = 表示するか
    }

    public 配色を設定する(配色: 道路点マーカー配色): void {
        this._材質束.配色を設定する(配色)
    }

    public 更新する(スプライン: 道路スプライン, 選択中の添字: number | null, つかんでいる添字: number | null): void {
        for (const マーカー of this._マーカー一覧) this.実体.remove(マーカー)
        this._マーカー一覧 = []

        for (let 添字 = 0; 添字 < スプライン.制御点列.length; 添字++) {
            const 点 = スプライン.制御点列[添字]
            if (点 === undefined) continue
            const 状態: 道路点マーカーの状態 =
                添字 === つかんでいる添字 ? 'つかんでいる' : 添字 === 選択中の添字 ? '選択中' : '通常'
            const マーカー = this._一点ぶんのマーカーを作る(状態)
            マーカー.position.set(点.x, 点.y + 地表から浮かせるメートル, 点.z)
            this.実体.add(マーカー)
            this._マーカー一覧.push(マーカー)
        }
        this.見かけの大きさを更新する()
    }

    // カメラから遠い点ほど世界での大きさを増やし、画面上の見かけの大きさをそろえる。
    public 見かけの大きさを更新する(): void {
        if (!this.実体.visible) return
        const カメラ位置 = this._カメラ.実体.getWorldPosition(this._カメラ位置の作業用ベクトル)
        for (const マーカー of this._マーカー一覧) {
            const 距離 = マーカー.position.distanceTo(カメラ位置)
            const 大きさ = Math.min(Math.max(距離 * カメラ距離に対する見かけの大きさ, 世界での最小の大きさ), 世界での最大の大きさ)
            マーカー.scale.setScalar(大きさ)
        }
    }

    private _一点ぶんのマーカーを作る(状態: 道路点マーカーの状態): Group {
        const 材質 = this._材質束.状態の材質を取り出す(状態)
        const マーカー = new Group()
        // 奥行き判定を切った物体は描画順で前後が決まるため、地形より後に描かれるよう並びを最後尾へ寄せる。
        const 輪 = new Mesh(this._輪ジオメトリ.実体, 材質)
        const 中心 = new Mesh(this._中心ジオメトリ.実体, 材質)
        輪.renderOrder = 10
        中心.renderOrder = 10
        マーカー.add(輪, 中心)
        return マーカー
    }
}
