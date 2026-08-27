import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import {
    パターンのステップ数,
    type トラックの格子,
    type トラック定義,
    type 和音,
} from '../../../生成/編集資源契約.ts'
import {
    数値からセルへ変換する,
    トラックの行の音はステップで許されるか,
} from '../編集モデル/index.ts'
import { 格子枠, 行枠, 行見出し, 升目列 } from './スタイル.css.ts'
import { 打ち込み升目部品 } from './打ち込み升目部品.ts'
import { トラック行の表示名 } from './音名表示.ts'

// 1つのトラックの全行・全ステップ（パターンのステップ数）の升目を並べる部品。
export class トラック格子部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _行升目一覧: 打ち込み升目部品[][] = []

    public constructor(トラック: トラック定義) {
        super()
        this._componentRoot = div({ class: 格子枠 })
        const 行数 = トラック.音の並び.値.length

        for (let 行位置 = 0; 行位置 < 行数; 行位置++) {
            const 表示名 = トラック行の表示名(トラック.音の並び, 行位置)
            const 行見出し要素 = div({ class: 行見出し, text: 表示名 }).setTooltip(表示名)
            const 升目一覧: 打ち込み升目部品[] = []

            for (let ステップ = 0; ステップ < パターンのステップ数; ステップ++) {
                升目一覧.push(new 打ち込み升目部品(ステップ))
            }
            this._行升目一覧.push(升目一覧)

            const 升目列要素 = div({ class: 升目列 }).childs(升目一覧)
            const 行要素 = div({ class: 行枠 }).childs([行見出し要素, 升目列要素])
            this._componentRoot.child(行要素)
        }
    }

    public 表示を更新する(
        トラック: トラック定義,
        トラック格子: トラックの格子,
        和音一覧: readonly 和音[] | null,
    ): void {
        const 行数 = トラック.音の並び.値.length
        for (let 行位置 = 0; 行位置 < 行数; 行位置++) {
            const 行データ = トラック格子.行一覧[行位置]
            const 升目一覧 = this._行升目一覧[行位置]
            if (行データ === undefined || 升目一覧 === undefined) continue

            for (let ステップ = 0; ステップ < パターンのステップ数; ステップ++) {
                const 生値 = 行データ[ステップ]
                const 升目部品 = 升目一覧[ステップ]
                if (生値 === undefined || 升目部品 === undefined) continue

                const 対象セル = 数値からセルへ変換する(生値)
                const 許されるか = トラックの行の音はステップで許されるか(
                    トラック,
                    行位置,
                    ステップ,
                    和音一覧,
                )
                升目部品.表示を更新する(対象セル, 許されるか)
            }
        }
    }
}
