import { div, span, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 楽曲 } from '../../../生成/編集資源契約.ts'
import type { 楽曲ID } from '../../../境界/index.ts'
import {
    コード進行参照から和音一覧を解決する,
    トラックに適用される和音一覧を解決する,
} from '../編集モデル/index.ts'
import { 永続化パネル } from '../../チャンク編集/画面/パネル/永続化/index.ts'
import {
    コンテナ,
    本文幅,
    ヘッダー行,
    タイトル,
    情報バッジ群,
    情報バッジ,
    エディター領域,
} from './スタイル.css.ts'
import { 進行の帯部品 } from './進行の帯部品.ts'
import { トラックブロック部品 } from './トラックブロック部品.ts'

// 楽曲エディター文書タブの画面全体。進行の帯・トラック格子群・永続化パネルを束ねる。
export class 楽曲編集画面 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 進行の帯: 進行の帯部品 = new 進行の帯部品()
    public readonly 永続化: 永続化パネル = new 永続化パネル()
    private readonly _タイトル要素: DivC
    private readonly _拍毎分バッジ: DivC
    private readonly _パターンバッジ: DivC
    private readonly _トラック領域: DivC
    private _トラックブロック一覧: トラックブロック部品[] = []

    public constructor(楽曲ID: 楽曲ID) {
        super()
        this._タイトル要素 = div({ class: タイトル, text: 楽曲ID })
        this._拍毎分バッジ = div({ class: 情報バッジ, text: 'BPM: -' })
        this._パターンバッジ = div({ class: 情報バッジ, text: 'パターン: -' })
        this._トラック領域 = div({ class: エディター領域 })

        this._componentRoot = div({ class: コンテナ }).child(
            div({ class: 本文幅 }).childs([
                div({ class: ヘッダー行 }).childs([
                    this._タイトル要素,
                    div({ class: 情報バッジ群 }).childs([
                        this._拍毎分バッジ,
                        this._パターンバッジ,
                    ]),
                ]),
                this.進行の帯,
                this._トラック領域,
                this.永続化,
            ]),
        )
    }

    public 表示を更新する(楽曲: 楽曲, 選択中パターン名乗り: string | null): void {
        const 表示名 = `${楽曲.表示名} (${楽曲.名乗り})`
        this._タイトル要素.clearChildren().child(span({ text: 表示名 })).setTooltip(表示名)
        this._拍毎分バッジ.clearChildren().child(span({ text: `BPM: ${楽曲.拍毎分}` }))

        const パターン = 選択中パターン名乗り === null
            ? 楽曲.パターン一覧[0]
            : 楽曲.パターン一覧.find((p) => p.名乗り === 選択中パターン名乗り)

        if (パターン === undefined) {
            this._パターンバッジ.clearChildren().child(span({ text: 'パターン: なし' }))
            this._トラック領域.clearChildren()
            return
        }

        this._パターンバッジ.clearChildren().child(span({ text: `パターン: ${パターン.表示名}` }))
        const パターン和音一覧 = コード進行参照から和音一覧を解決する(パターン.進行の参照, 楽曲.独自進行一覧)
        this.進行の帯.表示を更新する(パターン和音一覧)

        this.トラックブロック一覧を同期する(楽曲)
        for (let 位置 = 0; 位置 < 楽曲.トラック構成.length; 位置++) {
            const トラック = 楽曲.トラック構成[位置]
            const トラック格子 = パターン.格子[位置]
            const ブロック = this._トラックブロック一覧[位置]
            if (トラック === undefined || トラック格子 === undefined || ブロック === undefined) continue

            const トラック和音一覧 = トラックに適用される和音一覧を解決する(
                トラック,
                パターン.進行の参照,
                楽曲.独自進行一覧,
            )
            ブロック.表示を更新する(トラック, トラック格子, トラック和音一覧)
        }
    }

    private トラックブロック一覧を同期する(楽曲: 楽曲): void {
        if (this._トラックブロック一覧.length === 楽曲.トラック構成.length) return
        for (const ブロック of this._トラックブロック一覧) ブロック.delete()
        this._トラック領域.clearChildren()
        this._トラックブロック一覧 = 楽曲.トラック構成.map((トラック) => {
            const ブロック = new トラックブロック部品(トラック)
            this._トラック領域.child(ブロック)
            return ブロック
        })
    }

    public override delete(): void {
        this.進行の帯.delete()
        this.永続化.delete()
        for (const ブロック of this._トラックブロック一覧) ブロック.delete()
        super.delete()
    }
}
