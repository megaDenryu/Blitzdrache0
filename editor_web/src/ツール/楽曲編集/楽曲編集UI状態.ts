import type { 曲の節 } from '../../生成/編集資源契約.ts'
import type { 打ち込みドラッグ見込み } from './画面/打ち込み見込み.ts'
import type { カード位置 } from './編集モデル/index.ts'
export type { 打ち込みドラッグ見込み } from './画面/打ち込み見込み.ts'

// 楽曲エディターのUIセッション状態。進行の外モードやドラッグ中の見込み、
// タイムラインで選択中のカードを保持する。保存にもコマンドにも入らない(設計正本の判断15)。
export class 楽曲編集UI状態 {
    public 進行の外モードか: boolean = false
    public ドラッグ見込み: 打ち込みドラッグ見込み | null = null
    public 選択中のカード: カード位置 | null = null

    // コマンドの適用で曲構成が変わり、選択中のカードが指す先が無くなったら未選択へ戻す。
    public 選択中のカードが失われていたら外す(曲構成: readonly 曲の節[]): void {
        if (this.選択中のカード === null) return
        const 節 = 曲構成[this.選択中のカード.節の位置]
        if (節 === undefined || this.選択中のカード.繰り返しの何回目 >= 節.繰り返し回数) {
            this.選択中のカード = null
        }
    }
}

