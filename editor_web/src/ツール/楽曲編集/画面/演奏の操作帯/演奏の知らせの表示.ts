import { SpanC } from 'sengen-ui'
import type { 演奏の知らせ } from '../演奏/index.ts'
import { 知らせの帯 } from './スタイル.css.ts'

// 音を出せなかったことを利用者の言葉で見せる帯。伝えることが無い間は場所を取らずに消えている。
export class 演奏の知らせの表示 extends SpanC {
    private _映している文言: string = ''

    public constructor() {
        super({ class: 知らせの帯, text: '' })
        this.setAttribute('data-notice', 'false')
    }

    public 知らせを反映する(知らせ: 演奏の知らせ | null): this {
        const 文言 = 知らせ === null ? '' : 知らせ.文言
        if (this._映している文言 === 文言) return this
        this._映している文言 = 文言
        this.setTextContent(文言)
        this.setAttribute('data-notice', String(知らせ !== null))
        this.setTooltip(知らせ === null ? '' : `${知らせ.文言} (${知らせ.詳しい原因})`)
        return this
    }
}
