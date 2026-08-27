import { 世界エクスプローラー, 建物エクスプローラー, 楽曲エクスプローラー } from '../エクスプローラー/index.ts'
import type { タブ識別子 } from '../タブ識別子.ts'
import { タブ識別子から編集領域を見分ける } from './タブ識別子から編集領域を見分ける.ts'
import type { 編集領域ID } from './編集領域ID.ts'
import { 世界の編集領域, 建物の編集領域, 楽曲の編集領域 } from './編集領域の名乗り.ts'
import { 編集領域の登録 } from './編集領域の登録.ts'

// 編集領域の登録を3件そろえて保持し、識別子から登録を取り出す口と、前面のタブに合わせた
// 選択表示の振り分けを公開する操作サービス。3つのエクスプローラーの実体はこの型が所有する。
export class 編集領域登録簿 {
    private readonly _世界: 世界エクスプローラー = new 世界エクスプローラー()
    private readonly _建物: 建物エクスプローラー = new 建物エクスプローラー()
    private readonly _楽曲: 楽曲エクスプローラー = new 楽曲エクスプローラー()
    private readonly _登録一覧: readonly 編集領域の登録[]

    public constructor() {
        this._登録一覧 = [
            編集領域の登録.生成する(世界の編集領域, this._世界),
            編集領域の登録.生成する(建物の編集領域, this._建物),
            編集領域の登録.生成する(楽曲の編集領域, this._楽曲),
        ]
    }

    public get 登録一覧(): readonly 編集領域の登録[] {
        return this._登録一覧
    }

    public get 世界のエクスプローラー(): 世界エクスプローラー {
        return this._世界
    }

    public get 建物のエクスプローラー(): 建物エクスプローラー {
        return this._建物
    }

    public get 楽曲のエクスプローラー(): 楽曲エクスプローラー {
        return this._楽曲
    }

    public 識別子から登録を取り出す(識別子: 編集領域ID): 編集領域の登録 | null {
        return this._登録一覧.find((登録) => 登録.名乗り.識別子 === 識別子) ?? null
    }

    // 前面のタブが移ったとき、全領域の選択表示を消してから、そのタブが属する領域だけへ反映する。
    // タブは全領域で共有するため、選ばれていない領域の選択表示も同時に合わせる必要がある。
    public 前面のタブに合わせて選択表示する(タブ: タブ識別子): void {
        for (const 登録 of this._登録一覧) {
            登録.エクスプローラー.選択表示を解除する()
        }
        const 領域 = タブ識別子から編集領域を見分ける(タブ)
        if (領域 === null) return
        this.識別子から登録を取り出す(領域.識別子)?.エクスプローラー.前面のタブに合わせて選択表示する(タブ)
    }

    public 全て破棄する(): void {
        for (const 登録 of this._登録一覧) {
            登録.エクスプローラー.delete()
        }
    }
}
