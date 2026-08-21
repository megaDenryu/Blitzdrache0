import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { マテリアル定義 } from '../../../生成/編集資源契約.ts'
import { マテリアル定義行 } from './マテリアル定義行.ts'
import { 一覧コンテナ } from './スタイル.css.ts'

export interface Iマテリアル一覧配線 {
    readonly on名前変更: (添字: number, 新しい材質名: string) => void
    readonly on識別色変更: (添字: number, 新しい識別色: string) => void
    readonly on削除: (添字: number) => void
}

// マテリアル一覧の行を保持し、追加・削除のたびに丸ごと再構築するLV2素部品。
// 名前・識別色の編集は各行が自分のフィールドだけを持って完結するため、再構築しない。
export class マテリアル一覧部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private _行一覧: マテリアル定義行[] = []

    public constructor() {
        super()
        this._componentRoot = div({ class: 一覧コンテナ })
    }

    public 再構築する(一覧: ReadonlyArray<マテリアル定義>, 配線: Iマテリアル一覧配線): void {
        for (const 行 of this._行一覧) 行.delete()
        this._componentRoot.clearChildren()
        this._行一覧 = 一覧.map((定義, 添字) => {
            const 行 = new マテリアル定義行(定義).配線する({
                on名前変更: (新しい材質名) => 配線.on名前変更(添字, 新しい材質名),
                on識別色変更: (新しい識別色) => 配線.on識別色変更(添字, 新しい識別色),
                on削除: () => 配線.on削除(添字),
            })
            this._componentRoot.child(行)
            return 行
        })
    }

    public override delete(): void {
        for (const 行 of this._行一覧) 行.delete()
        super.delete()
    }
}
