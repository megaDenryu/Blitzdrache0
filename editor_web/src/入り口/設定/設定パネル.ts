import { div, span, DivC, SpanC, LV2HtmlComponentBase } from 'sengen-ui'
import type { テーマ管理サービス } from '../テーマ/テーマ管理サービス.ts'
import type { エディターテーマ定義, テーマ名 } from '../テーマ/テーマ型.ts'
import { テーマ定義一覧 } from '../テーマ/テーマ定義一覧.ts'
import { 代表色トークン一覧 } from './代表色トークン一覧.ts'
import {
    コンテナ,
    セクション見出し,
    テーマリスト,
    テーマカード,
    カードヘッダー,
    テーマ表示名,
    選択バッジ,
    テーマ説明,
    パレットプレビュー,
    色スウォッチ,
} from './スタイル.css.ts'

interface テーマカード要素群 {
    readonly カードコンテナ: DivC
    readonly バッジ要素: SpanC
}

// 左サイドバーに配置され、カラーテーマの切り替えを提供する設定パネル。
export class 設定パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _カード要素マップ: Map<テーマ名, テーマカード要素群> = new Map()
    private readonly _購読解除: () => void

    public constructor(private readonly _テーマサービス: テーマ管理サービス) {
        super()

        const カード一覧: DivC[] = []
        for (const テーマ of テーマ定義一覧) {
            const カード要素 = this._テーマカードを生成する(テーマ)
            this._カード要素マップ.set(テーマ.名前, カード要素)
            カード一覧.push(カード要素.カードコンテナ)
        }

        this._componentRoot = div({ class: コンテナ }).childs([
            div({ class: セクション見出し, text: '外観設定' }),
            div({ class: テーマリスト }).childs(カード一覧),
        ])

        this._選択状態を同期する(this._テーマサービス.現在テーマを取得する().名前)

        this._購読解除 = this._テーマサービス.onテーマ変更((新テーマ) => {
            this._選択状態を同期する(新テーマ.名前)
        })
    }

    private _テーマカードを生成する(テーマ: エディターテーマ定義): テーマカード要素群 {
        const 現在選択中 = this._テーマサービス.現在テーマを取得する().名前 === テーマ.名前

        const バッジ要素 = span({
            class: 選択バッジ,
            text: '適用中',
        }).setStyleCSS({ display: 現在選択中 ? 'inline-block' : 'none' })

        const スウォッチ一覧: DivC[] = []
        const 代表色一覧 = 代表色トークン一覧.map((トークン) => テーマ.エディター配色[トークン])

        for (const 色 of 代表色一覧) {
            スウォッチ一覧.push(
                div({ class: 色スウォッチ }).setStyleCSS({ backgroundColor: 色 })
            )
        }

        const カードコンテナ = div({ class: テーマカード })
            .setAttribute('data-selected', 現在選択中 ? 'true' : 'false')
            .childs([
                div({ class: カードヘッダー }).childs([
                    span({ class: テーマ表示名, text: テーマ.表示名 }),
                    バッジ要素,
                ]),
                span({ class: テーマ説明, text: テーマ.説明 }),
                div({ class: パレットプレビュー }).childs(スウォッチ一覧),
            ])
            .onClick(() => {
                this._テーマサービス.テーマを変更する(テーマ.名前)
            })

        return { カードコンテナ, バッジ要素 }
    }

    private _選択状態を同期する(選択中テーマ名: テーマ名): void {
        for (const [名前, 要素群] of this._カード要素マップ.entries()) {
            const 一致 = 名前 === 選択中テーマ名
            要素群.カードコンテナ.setAttribute('data-selected', 一致 ? 'true' : 'false')
            要素群.バッジ要素.setStyleCSS({ display: 一致 ? 'inline-block' : 'none' })
        }
    }

    public override delete(): void {
        this._購読解除()
        super.delete()
    }
}
