import { div, span, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 楽曲 } from '../../../生成/編集資源契約.ts'
import type { 楽曲ID } from '../../../境界/index.ts'
import {
    ヘッダー行,
    タイトル,
    情報バッジ群,
    情報バッジ,
} from './スタイル.css.ts'

// 楽曲エディター上部のヘッダー行。タイトル・BPM・選択中パターン・進行制約バッジを表示する。
export class 楽曲ヘッダー部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _タイトル要素: DivC
    private readonly _拍毎分バッジ: DivC
    private readonly _パターンバッジ: DivC
    private readonly _進行外バッジ: DivC

    public constructor(楽曲ID: 楽曲ID) {
        super()
        this._タイトル要素 = div({ class: タイトル, text: 楽曲ID })
        this._拍毎分バッジ = div({ class: 情報バッジ, text: 'BPM: -' })
        this._パターンバッジ = div({ class: 情報バッジ, text: 'パターン: -' })
        this._進行外バッジ = div({ class: 情報バッジ, text: '進行制約: 追従' })

        this._componentRoot = div({ class: ヘッダー行 }).childs([
            this._タイトル要素,
            div({ class: 情報バッジ群 }).childs([
                this._拍毎分バッジ,
                this._パターンバッジ,
                this._進行外バッジ,
            ]),
        ])
    }

    public 表示を更新する(
        楽曲: 楽曲,
        パターン表示名: string | null,
        進行の外モードか: boolean,
    ): void {
        const 表示名 = `${楽曲.表示名} (${楽曲.名乗り})`
        this._タイトル要素.clearChildren().child(span({ text: 表示名 })).setTooltip(表示名)
        this._拍毎分バッジ.clearChildren().child(span({ text: `BPM: ${楽曲.拍毎分}` }))
        this._進行外バッジ.clearChildren().child(
            span({ text: 進行の外モードか ? '進行制約: 進行の外' : '進行制約: 追従' }),
        )
        const パターン文言 = パターン表示名 !== null ? `パターン: ${パターン表示名}` : 'パターン: なし'
        this._パターンバッジ.clearChildren().child(span({ text: パターン文言 }))
    }
}
