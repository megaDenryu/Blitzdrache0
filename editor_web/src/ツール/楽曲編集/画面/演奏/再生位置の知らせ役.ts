import { 配線ポート } from 'sengen-ui'
import { 毎フレームの見張り, type I画面の1コマごとに起こされる側 } from './毎フレームの見張り.ts'
import type { I再生位置の届け先, 再生位置 } from './再生位置.ts'

// いまの再生位置を答えられる側の規約。演奏サービスが実装する。
export interface I再生位置を答えられる側 {
    現在の再生位置(): 再生位置 | null
}

// 音声の時計から導いた再生位置を、画面の1コマごとに届ける役。
// 印を出す仕事は音を出す仕事と進み方が違う(画面の速さで進む)ため、見張りと届け先の口をここが所有する。
export class 再生位置の知らせ役 implements I画面の1コマごとに起こされる側 {
    public readonly on再生位置が変わった: 配線ポート<I再生位置の届け先> =
        new 配線ポート<I再生位置の届け先>('再生位置の知らせ役')
    private readonly _見張り: 毎フレームの見張り = new 毎フレームの見張り(this)

    public constructor(private readonly _位置を答える相手: I再生位置を答えられる側) {}

    public 届け始める(): void {
        this._見張り.始める()
    }

    public 届けるのをやめる(): void {
        this._見張り.やめる()
    }

    public いまの位置を届ける(): void {
        if (!this.on再生位置が変わった.配線済みか) return
        this.on再生位置が変わった.先.再生位置が変わった(this._位置を答える相手.現在の再生位置())
    }

    public 画面が1コマ進んだ(): void {
        this.いまの位置を届ける()
    }
}
