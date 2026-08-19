import { div, span, p, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { パネル, 見出し行, バッジ, 説明リスト } from './スタイル.css.ts'

// 129×129の境界頂点共有(1画素重複)と道路クリップ仕様を案内するLV2素部品。
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
                    span({ text: 'チャンク切り出しの設定' }).setTooltip('チャンク切り出しの設定'),
                    span({ class: バッジ, text: '129×129 (1画素重複)' }).setTooltip('129×129 (1画素重複)'),
                ]),
                div({ class: 説明リスト }).childs([
                    p({ text: '・ 境界の頂点を隣と共有(1画素重複)して出力。' }),
                    p({ text: '・ 道路スプラインを各チャンクの外接範囲で自動クリップ。' }),
                ]),
            ])
        )
    }
}
