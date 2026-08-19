import { div, span, p, button, textInput, DivC, ButtonC, TextInputC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import {
    パネル, 見出し行, バッジ, 説明リスト,
    書き出し行, 世界名入力, 書き出しボタン, 書き出し状態メッセージ, 書き出しエラー状態メッセージ,
} from './スタイル.css.ts'

export interface Iスライス仕様パネル配線 {
    readonly on書き出しクリック: (世界名: string) => void
}

const 既定世界名 = 'editor_world'

// 129×129の境界頂点共有(1画素重複)と道路クリップ仕様を案内し、ソースアセットへの
// 書き出しボタンと出力先の世界名入力欄を持つLV2素部品。
export class スライス仕様パネル extends LV2HtmlComponentBase implements I配線可能<Iスライス仕様パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iスライス仕様パネル配線> = new 配線ポート<Iスライス仕様パネル配線>('スライス仕様パネル')
    private readonly _世界名入力: TextInputC
    private readonly _書き出しボタン: ButtonC
    private readonly _状態表示コンテナ: DivC

    public constructor() {
        super()
        this._世界名入力 = textInput({ class: 世界名入力, value: 既定世界名, placeholder: '世界名' })
            .setTooltip('書き出し先の世界名')
        this._書き出しボタン = button({ class: 書き出しボタン, text: 'ソースアセットへ書き出す' })
            .setTooltip('ソースアセットへ書き出す')
        this._状態表示コンテナ = div({ class: 書き出し状態メッセージ }).setTooltip('').child(span({ text: '' }))
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: Iスライス仕様パネル配線): this {
        this._配線.配線する(配線)
        this._書き出しボタン.onClick(() => {
            this._配線.先.on書き出しクリック(this._世界名を取得する())
        })
        return this
    }

    public ボタン活性状態を更新する(活性: boolean): void {
        this._書き出しボタン.setStyleCSS({
            opacity: 活性 ? '1' : '0.5',
            pointerEvents: 活性 ? 'auto' : 'none',
        })
    }

    public 状態文言を更新する(文言: string, エラーか: boolean = false): void {
        this._状態表示コンテナ.setTooltip(文言).clearChildren()
        this._状態表示コンテナ.child(
            span({ class: エラーか ? 書き出しエラー状態メッセージ : 書き出し状態メッセージ, text: 文言 }),
        )
    }

    public override delete(): void {
        this._世界名入力.delete()
        this._書き出しボタン.delete()
        this._状態表示コンテナ.delete()
        super.delete()
    }

    private _世界名を取得する(): string {
        const 値 = this._世界名入力.getValue().trim()
        return 値 === '' ? 既定世界名 : 値
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                div({ class: 見出し行 }).childs([
                    span({ text: 'チャンク切り出しの設定' }).setTooltip('チャンク切り出しの設定'),
                    span({ class: バッジ, text: '129×129 (1画素重複)' }).setTooltip('129×129 (1画素重複)'),
                ]),
                div({ class: 説明リスト }).childs([
                    p({ text: '・ 境界の頂点を隣と共有(1画素重複)して出力。' }),
                    p({ text: '・ 道路スプラインを各チャンクの外接範囲で自動クリップ。' }),
                ]),
                div({ class: 書き出し行 }).childs([this._世界名入力, this._書き出しボタン]),
                this._状態表示コンテナ,
            ])
        )
    }
}
