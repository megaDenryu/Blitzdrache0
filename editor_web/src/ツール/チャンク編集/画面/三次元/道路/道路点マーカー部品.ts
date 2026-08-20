import { CircleGeometry, Object3D, RingGeometry, Vector3 } from 'three'
import { グループ, ジオメトリ包み } from 'SengenThree'
import type { 透視カメラ } from 'SengenThree'
import type { 道路の一覧, 道路点の在り処 } from '../../../編集モデル/index.ts'
import { 道路点マーカー材質束, type 道路点マーカー配色 } from './道路点マーカー材質束.ts'
import { 道路点マーカー一覧 } from './道路点マーカー一覧.ts'
import type { 道路点マーカーの表示の指定 } from './道路点マーカーの見た目の状態.ts'

export type { 道路点マーカーの表示の指定 }

// 道路一覧の全ての制御点の位置に、地表へ伏せた輪と中心点のマーカーを並べる部品。
// 形状と材質の資源をここが所有し、マーカーの実体の置き直しと当たり判定は道路点マーカー一覧が持つ。
// 見かけの大きさをカメラ距離へ追従させるため、毎フレーム`見かけの大きさを更新する`を呼ぶ。
export class 道路点マーカー部品 extends グループ {
    private readonly _輪ジオメトリ: ジオメトリ包み
    private readonly _中心ジオメトリ: ジオメトリ包み
    private readonly _材質束: 道路点マーカー材質束
    private readonly _マーカー一覧: 道路点マーカー一覧
    private readonly _カメラ位置の作業用ベクトル = new Vector3()

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
        this._マーカー一覧 = new 道路点マーカー一覧(
            this.実体,
            this._輪ジオメトリ.実体,
            this._中心ジオメトリ.実体,
            this._材質束,
        )

        this.資源台帳.登録する(this._輪ジオメトリ)
        this.資源台帳.登録する(this._中心ジオメトリ)
        for (const 材質 of this._材質束.材質一覧) this.資源台帳.登録する(材質)
    }

    // 交差した物体からその物体が属する道路点の在り処を求める。当たっていなければnullを返す。
    public 当たった道路点の在り処を求める(交差した物体: Object3D): 道路点の在り処 | null {
        return this._マーカー一覧.当たった道路点の在り処を求める(交差した物体)
    }

    public 表示するか設定する(表示するか: boolean): void {
        this.実体.visible = 表示するか
    }

    public 配色を設定する(配色: 道路点マーカー配色): void {
        this._材質束.配色を設定する(配色)
    }

    public 更新する(道路一覧: 道路の一覧, 指定: 道路点マーカーの表示の指定): void {
        this._マーカー一覧.置き直す(道路一覧, 指定)
        this.見かけの大きさを更新する()
    }

    public 見かけの大きさを更新する(): void {
        if (!this.実体.visible) return
        this._マーカー一覧.見かけの大きさを合わせる(this._カメラ.実体.getWorldPosition(this._カメラ位置の作業用ベクトル))
    }
}
