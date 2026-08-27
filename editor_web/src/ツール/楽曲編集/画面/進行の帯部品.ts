import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import {
    パターンのステップ数,
    type 和音,
} from '../../../生成/編集資源契約.ts'
import {
    進行の帯の区切り一覧を計算する,
    和音の表示名を組み立てる,
} from '../編集モデル/index.ts'
import {
    進行の帯枠,
    進行見出し余白,
    進行和音列,
    進行和音ブロック,
} from './スタイル.css.ts'

// パターンまたはトラックのコード進行を格子のステップ幅に合わせて帯状に表示する部品。
export class 進行の帯部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _和音列コンテナ: DivC

    public constructor() {
        super()
        this._和音列コンテナ = div({ class: 進行和音列 })
        this._componentRoot = div({ class: 進行の帯枠 }).childs([
            div({ class: 進行見出し余白, text: 'コード進行' }),
            this._和音列コンテナ,
        ])
    }

    public 表示を更新する(和音一覧: readonly 和音[]): void {
        this._和音列コンテナ.clearChildren()
        const 区切り一覧 = 進行の帯の区切り一覧を計算する(和音一覧, パターンのステップ数)

        for (const 区切り of 区切り一覧) {
            const 表示名 = 和音の表示名を組み立てる(区切り.和音)
            const ラベル = `${表示名} (${区切り.ステップ幅})`
            const ブロック = div({ class: 進行和音ブロック, text: ラベル })
                .setStyleCSS({ flex: String(区切り.ステップ幅) })
                .setTooltip(`${表示名} (${区切り.ステップ幅}ステップ${区切り.周回後か ? '・周回' : ''})`)

            if (区切り.周回後か) {
                ブロック.setAttribute('data-repeated', 'true')
            }
            this._和音列コンテナ.child(ブロック)
        }
    }
}
