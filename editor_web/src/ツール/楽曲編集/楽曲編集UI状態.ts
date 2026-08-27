import type { 打ち込みドラッグ見込み } from './画面/打ち込み見込み.ts'
export type { 打ち込みドラッグ見込み } from './画面/打ち込み見込み.ts'

// 楽曲エディターのUIセッション状態。進行の外モードやドラッグ中の見込みを保持する。
export class 楽曲編集UI状態 {
    public 進行の外モードか: boolean = false
    public ドラッグ見込み: 打ち込みドラッグ見込み | null = null
}

