import { 配線ポート } from 'sengen-ui'
import {
    音を出せなかった知らせを組み立てる,
    type I演奏の知らせの届け先,
    type 演奏の知らせ,
} from './演奏の知らせ.ts'

// 演奏で起きたことがらを画面へ運ぶ口。届け先がまだ配線されていない間は、伝えるべき相手がいないので黙る。
export class 演奏の知らせの口 {
    public readonly on演奏の知らせ: 配線ポート<I演奏の知らせの届け先> =
        new 配線ポート<I演奏の知らせの届け先>('演奏の知らせの口')

    public 音を出せなかったことを伝える(原因: unknown): void {
        this._配る(音を出せなかった知らせを組み立てる(原因))
    }

    // 新しく押し直したときに、前の失敗の文言を残さないための消去。
    public 伝えることを消す(): void {
        this._配る(null)
    }

    private _配る(知らせ: 演奏の知らせ | null): void {
        if (!this.on演奏の知らせ.配線済みか) return
        this.on演奏の知らせ.先.演奏の知らせが届いた(知らせ)
    }
}
