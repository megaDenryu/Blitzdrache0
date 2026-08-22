import type { 建物の入口方向 } from '../../../生成/編集資源契約.ts'

// 建物1件が升目の並びとは別に持つ名乗り。識別子と表示名と入口の方向である。
// 升目の表と分けるのは、触る材料が違うためである。升目の表は座標の近さと継ぎ目を見て決まり、
// 名乗りは升目が1つも変わらなくても人が変えられる。
export class 建物の名乗り {
    public constructor(
        public readonly 建物定義ID: string,
        private _表示名: string,
        private _入口のローカル方向: 建物の入口方向,
    ) {}

    public get 表示名(): string {
        return this._表示名
    }

    public get 入口のローカル方向(): 建物の入口方向 {
        return this._入口のローカル方向
    }

    public 表示名を定める(表示名: string): void {
        this._表示名 = 表示名
    }

    public 入口の方向を定める(方向: 建物の入口方向): void {
        this._入口のローカル方向 = 方向
    }
}
