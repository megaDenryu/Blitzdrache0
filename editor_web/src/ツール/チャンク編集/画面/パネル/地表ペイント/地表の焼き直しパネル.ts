import { div, span, button, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 道路の泥の追従方針 } from '../../../編集モデル/index.ts'
import { 道路の泥の追従切替 } from '../共通/道路の泥の追従切替.ts'
import { パネル, 見出し, ベイク区画, アクションボタン } from './スタイル.css.ts'

export interface I地表の焼き直しパネル配線 {
    readonly on急勾配ベイク: () => void
    readonly on道路下泥ベイク: () => void
    readonly on道路の泥の追従方針変更: (方針: 道路の泥の追従方針) => void
}

// チャンク全体の地表材質を機械の規則で焼き直す操作をまとめたLV2素部品。右サイドバーへ置く。
// 塗る材質を選ぶ筆と分けてあるのは、こちらが「これから使う道具」ではなく、
// いま開いているチャンクへ効く設定と操作だからである(設計正本の判断14)。
export class 地表の焼き直しパネル extends LV2HtmlComponentBase implements I配線可能<I地表の焼き直しパネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I地表の焼き直しパネル配線> = new 配線ポート<I地表の焼き直しパネル配線>('地表の焼き直しパネル')
    private readonly _泥の追従切替: 道路の泥の追従切替

    public constructor(初期の泥の追従方針: 道路の泥の追従方針) {
        super()
        this._泥の追従切替 = new 道路の泥の追従切替(初期の泥の追従方針)
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I地表の焼き直しパネル配線): this {
        this._配線.配線する(配線)
        this._泥の追従切替.切替時((方針) => this._配線.先.on道路の泥の追従方針変更(方針))
        return this
    }

    public override delete(): void {
        this._泥の追従切替.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                span({ class: 見出し, text: '地表の焼き直し' }).setTooltip('地表の焼き直し'),
                div({ class: ベイク区画 }).childs([
                    button({ class: アクションボタン, text: '急勾配(>30度)を自動で岩肌にベイク' })
                        .setTooltip('急勾配(>30度)を自動で岩肌にベイク')
                        .onClick(() => this._配線.先.on急勾配ベイク()),
                    this._泥の追従切替,
                    button({ class: アクションボタン, text: '道路下の泥を焼き直す' })
                        .setTooltip('道路下の泥を焼き直す')
                        .onClick(() => this._配線.先.on道路下泥ベイク())])])
        )
    }
}
