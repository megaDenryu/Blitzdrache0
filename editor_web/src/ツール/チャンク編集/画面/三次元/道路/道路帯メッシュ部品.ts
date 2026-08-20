import type { Object3D } from 'three'
import { グループ } from 'SengenThree'
import type { 道路の一覧, 高さ場 } from '../../../編集モデル/index.ts'
import { 道路1本の帯メッシュ部品 } from './道路1本の帯メッシュ部品.ts'

// 道路一覧のすべての道路の帯を表示する部品。1本につき1つの子部品を持ち、本数の増減に合わせて
// 子部品を作り足し、余った子部品は破棄する(破棄で形状と材質の資源も解放される)。
export class 道路帯メッシュ部品 extends グループ {
    private _帯一覧: 道路1本の帯メッシュ部品[] = []
    private _道路色: number

    public constructor(初期道路色: number) {
        super()
        this._道路色 = 初期道路色
    }

    public 更新する(道路一覧: 道路の一覧, 高さ場モデル: 高さ場): void {
        this._本数を合わせる(道路一覧.件数)
        道路一覧.全ての道路.forEach((道路, 添字) => {
            this._帯一覧[添字]?.更新する(道路, 高さ場モデル)
        })
    }

    // 交差した物体が路面なら、その路面が属する道路の添字を返す。路面でなければnullを返す。
    public 当たった路面の道路添字を求める(交差した物体: Object3D): number | null {
        for (let 添字 = 0; 添字 < this._帯一覧.length; 添字++) {
            if (this._帯一覧[添字]?.路面メッシュか(交差した物体) === true) return 添字
        }
        return null
    }

    // テーマ切替時に全ての道路の路面の色を差し替える。後から作る帯にも同じ色を渡すため色を覚えておく。
    public 道路色を更新する(道路色: number): void {
        this._道路色 = 道路色
        for (const 帯 of this._帯一覧) 帯.道路色を更新する(道路色)
    }

    private _本数を合わせる(本数: number): void {
        while (this._帯一覧.length > 本数) {
            this._帯一覧.pop()?.破棄する()
        }
        while (this._帯一覧.length < 本数) {
            const 帯 = new 道路1本の帯メッシュ部品(this._道路色)
            this.child(帯)
            this._帯一覧.push(帯)
        }
    }
}
