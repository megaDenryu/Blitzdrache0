import { SpanC } from 'sengen-ui'
import type { 下書きと正本の揃い } from '../../../編集モデル/index.ts'
import { 揃いの札 } from './スタイル.css.ts'

// 4つの状態を人が読む文言へ写す。生成の直後は「揃っている」、下書きを触ると「下書きが新しい」、
// 三次元の筆で正本を触ると「正本が新しい」になる(設計正本の操作契約)。
const 揃いの文言: Record<下書きと正本の揃い, string> = {
    未確認: '下書きと地形の揃い: 未確認(読み込んだ直後)',
    揃っている: '下書きと地形の揃い: 揃っている',
    下書きが新しい: '下書きと地形の揃い: 下書きが新しい(生成すると地形へ反映される)',
    正本が新しい: '下書きと地形の揃い: 正本が新しい(導くと下書きへ反映される)',
}

// 下書きと正本の揃いを1行で出す札(LV1拡張)。等高線パネルと大升パネルの両方が持ち、同じ値を出す。
export class 揃いの表示 extends SpanC {
    public constructor() {
        super({ class: 揃いの札 })
        this.揃いを更新する('未確認')
    }

    public 揃いを更新する(揃い: 下書きと正本の揃い): this {
        const 文言 = 揃いの文言[揃い]
        this.setTextContent(文言)
        this.setTooltip(文言)
        return this
    }
}
