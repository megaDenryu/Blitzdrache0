import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { トラックの格子, トラック定義, 和音 } from '../../../生成/編集資源契約.ts'
import { トラック枠 } from './スタイル.css.ts'
import { トラック見出し部品 } from './トラック見出し部品.ts'
import { トラック格子部品 } from './トラック格子部品.ts'

// 1つのトラックの見出しと格子を束ねる部品。
export class トラックブロック部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 見出し: トラック見出し部品
    public readonly 格子: トラック格子部品

    public constructor(トラック: トラック定義) {
        super()
        this.見出し = new トラック見出し部品(トラック)
        this.格子 = new トラック格子部品(トラック)
        this._componentRoot = div({ class: トラック枠 }).childs([this.見出し, this.格子])
    }

    public 表示を更新する(
        トラック: トラック定義,
        トラック格子: トラックの格子,
        和音一覧: readonly 和音[] | null,
    ): void {
        this.見出し.表示を更新する(トラック)
        this.格子.表示を更新する(トラック, トラック格子, 和音一覧)
    }

    public override delete(): void {
        this.見出し.delete()
        this.格子.delete()
        super.delete()
    }
}
