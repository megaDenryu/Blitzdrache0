import type { I演奏の知らせの届け先, 演奏の知らせ, 楽曲編集画面 } from './画面/index.ts'

// 演奏で起きたことがらを、演奏の操作帯の知らせの帯へ映す。
export class 演奏の知らせの反映 implements I演奏の知らせの届け先 {
    public constructor(private readonly _画面: 楽曲編集画面) {}

    public 演奏の知らせが届いた(知らせ: 演奏の知らせ | null): void {
        this._画面.操作帯.演奏の知らせを反映する(知らせ)
    }
}
