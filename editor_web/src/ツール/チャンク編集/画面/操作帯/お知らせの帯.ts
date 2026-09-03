import { DivC } from 'sengen-ui'
import { お知らせの帯の枠 } from './お知らせの帯.css.ts'

// 操作帯の下に出す1行の文言の帯(LV1拡張)。生成できないときの理由など、操作の結果をここへ出す
// (設計正本の操作契約「生成できないときは理由を操作帯の下の帯へ出す」)。文言が無いときは行ごと隠す。
export class お知らせの帯 extends DivC {
    public constructor() {
        super({ class: お知らせの帯の枠 })
        this.消す()
    }

    public 文言を出す(文言: string): void {
        this.setTextContent(文言)
        this.setTooltip(文言)
        this.setStyleCSS({ display: '' })
    }

    public 消す(): void {
        this.setTextContent('')
        this.setStyleCSS({ display: 'none' })
    }
}
