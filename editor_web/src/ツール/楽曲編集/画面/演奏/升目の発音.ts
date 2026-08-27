import type { I楽曲発音配線 } from '../発音配線.ts'
import type { 演奏サービス } from './演奏サービス.ts'

// 升目を押したときの発音の口を、演奏サービスの1音の発音へつなぐ薄い層。
export class 升目の発音 implements I楽曲発音配線 {
    public constructor(private readonly _演奏: 演奏サービス) {}

    public 升目の音を鳴らす(トラックの位置: number, 行の位置: number): void {
        this._演奏.升目の音を1回鳴らす(トラックの位置, 行の位置)
    }
}
