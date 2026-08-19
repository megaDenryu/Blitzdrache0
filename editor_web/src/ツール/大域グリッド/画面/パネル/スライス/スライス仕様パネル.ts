import { div, span, p, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { パネル, 見出し行, バッジ, 説明リスト } from './スタイル.css.ts'

// 129x129の1px重複共有と道路クリップ仕様を案内するLV2素部品。
export class スライス仕様パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC

    public constructor() {
        super()
        this._componentRoot = this._ルートを構築する()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                div({ class: 見出し行 }).childs([
                    span({ text: 'シームレス・スライス設定' }).setTooltip('シームレス・スライス設定'),
                    span({ class: バッジ, text: '129x129 (1px Overlap)' }).setTooltip('129x129 (1px Overlap)'),
                ]),
                div({ class: 説明リスト }).childs([
                    p({ text: '・ 各チャンク境界を1px重複共有して出力。' }),
                    p({ text: '・ 道路スプラインを各チャンクのAABBで自動クリップ。' }),
                ]),
            ])
        )
    }
}
