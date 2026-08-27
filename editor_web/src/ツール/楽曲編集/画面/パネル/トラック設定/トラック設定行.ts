import { div, span, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import {
    音量と効果の比の下限,
    音量と効果の比の上限,
    type トラック定義,
    type 楽器,
    type コード進行参照,
    type コード進行,
} from '../../../../../生成/編集資源契約.ts'
import { スライダー項目 } from '../共通/スライダー項目.ts'
import { 行コンテナ, 項目ラベル } from '../共通/スタイル.css.ts'
import { トラックの楽器選択欄 } from './トラックの楽器選択欄.ts'
import { トラックの進行選択欄 } from './トラックの進行選択欄.ts'
import { トラック設定行枠, トラック見出し行, トラック名, 種類バッジ, トラック項目群 } from './スタイル.css.ts'

const 音量の刻み幅 = 0.05

export interface Iトラック設定行配線 {
    readonly on楽器変更: (新しい楽器: 楽器) => void
    readonly on音量変更: (新しい音量: number) => void
    readonly on進行割り当て変更: (新しい進行の割り当て: コード進行参照 | null) => void
}

// トラック1本の楽器・音量・進行割り当ての入力コンポーネント。
export class トラック設定行 extends LV2HtmlComponentBase implements I配線可能<Iトラック設定行配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iトラック設定行配線> = new 配線ポート<Iトラック設定行配線>('トラック設定行')
    private readonly _楽器選択: トラックの楽器選択欄
    private readonly _音量スライダー: スライダー項目
    private readonly _進行選択: トラックの進行選択欄

    public constructor(トラック: トラック定義, 独自進行一覧: readonly コード進行[]) {
        super()
        this._楽器選択 = new トラックの楽器選択欄(トラック)
        this._音量スライダー = new スライダー項目(
            '音量',
            音量と効果の比の下限,
            音量と効果の比の上限,
            音量の刻み幅,
            トラック.音量,
        )
        this._進行選択 = new トラックの進行選択欄(トラック, 独自進行一覧)
        this._componentRoot = this._ルートを構築する(トラック)
    }

    public 配線する(配線: Iトラック設定行配線): this {
        this._配線.配線する(配線)
        this._楽器選択.onSelectChange(() => this._楽器の選び直しを伝える())
        this._音量スライダー.配線する({
            on値変更: (新音量) => {
                if (this._配線.配線済みか) this._配線.先.on音量変更(新音量)
            },
        })
        this._進行選択.onSelectChange(() => this._進行の選び直しを伝える())
        return this
    }

    public 表示を更新する(トラック: トラック定義, 独自進行一覧: readonly コード進行[]): void {
        this._楽器選択.トラックを反映する(トラック)
        this._進行選択.トラックを反映する(トラック, 独自進行一覧)
        this._音量スライダー.値を設定する(トラック.音量)
    }

    public override delete(): void {
        this._楽器選択.delete()
        this._音量スライダー.delete()
        this._進行選択.delete()
        super.delete()
    }

    private _楽器の選び直しを伝える(): void {
        if (this._配線.配線済みか) this._配線.先.on楽器変更(this._楽器選択.選ばれた楽器())
    }

    private _進行の選び直しを伝える(): void {
        if (this._配線.配線済みか) this._配線.先.on進行割り当て変更(this._進行選択.選ばれた進行の割り当て())
    }

    private _ルートを構築する(トラック: トラック定義): DivC {
        return div({ class: トラック設定行枠 }).childs([
            div({ class: トラック見出し行 }).childs([
                span({ class: トラック名, text: トラック.表示名 }),
                span({ class: 種類バッジ, text: トラック.種類 }),
            ]),
            div({ class: トラック項目群 }).childs([
                div({ class: 行コンテナ }).childs([
                    span({ class: 項目ラベル, text: '楽器' }),
                    this._楽器選択,
                ]),
                this._音量スライダー,
                div({ class: 行コンテナ }).childs([
                    span({ class: 項目ラベル, text: '進行の割り当て' }),
                    this._進行選択,
                ]),
            ]),
        ])
    }
}
