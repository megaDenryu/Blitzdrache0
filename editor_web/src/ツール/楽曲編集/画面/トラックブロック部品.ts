import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { トラックの格子, トラック定義, 和音 } from '../../../生成/編集資源契約.ts'
import type { 打ち込みドラッグ見込み, 升目の当たりの記録 } from './打ち込み見込み.ts'
import { トラック枠 } from './スタイル.css.ts'

import { トラック見出し部品 } from './トラック見出し部品.ts'
import { トラック格子部品 } from './トラック格子部品.ts'

// 1つのトラックの見出しと格子を束ねる部品。
export class トラックブロック部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 見出し: トラック見出し部品
    public readonly 格子: トラック格子部品

    public constructor(
        トラック: トラック定義,
        トラックの位置: number,
        パターンの名乗り取得: () => string,
        on升目押下: (当たり: 升目の当たりの記録, ボタン: number) => void,
        on升目進入: (当たり: 升目の当たりの記録) => void,
        初期ステップ数: number,
    ) {
        super()
        this.見出し = new トラック見出し部品(トラック)
        this.格子 = new トラック格子部品(トラック, トラックの位置, パターンの名乗り取得, on升目押下, on升目進入, 初期ステップ数)
        this._componentRoot = div({ class: トラック枠 }).childs([this.見出し, this.格子])
    }

    public 再生位置を示す(ステップ: number | null): void {
        this.格子.再生位置を示す(ステップ)
    }

    public 強調する小節を示す(小節の番号: number | null): void {
        this.格子.強調する小節を示す(小節の番号)
    }

    public 表示を更新する(
        トラック: トラック定義,
        トラック格子: トラックの格子,
        和音一覧: readonly 和音[] | null,
        パターンの名乗り: string,
        トラックの位置: number,
        ドラッグ見込み: 打ち込みドラッグ見込み | null,
        ステップ数: number,
    ): void {
        this.見出し.表示を更新する(トラック)
        this.格子.表示を更新する(トラック, トラック格子, 和音一覧, パターンの名乗り, トラックの位置, ドラッグ見込み, ステップ数)
    }

    public override delete(): void {
        this.見出し.delete()
        this.格子.delete()
        super.delete()
    }
}
