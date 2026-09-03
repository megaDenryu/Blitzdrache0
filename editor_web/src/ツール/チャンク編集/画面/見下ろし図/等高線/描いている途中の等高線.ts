import type { 等高線, 平面の位置 } from '../../../../../生成/編集資源契約.ts'

// 等高線モードで、左クリックで頂点を置き始めてから線を終えるまでの頂点列。確定前であり編集モデルには無い。
// 1本描き終えるまでが1コマンドであるため(設計正本の判断2)、確定したときに初めて契約の形へ写す。
export class 描いている途中の等高線 {
    private _頂点列: 平面の位置[] = []

    public get 頂点列(): readonly 平面の位置[] {
        return this._頂点列
    }

    public get 描いているか(): boolean {
        return this._頂点列.length > 0
    }

    public 頂点を足す(位置: 平面の位置): void {
        this._頂点列.push({ x: 位置.x, z: 位置.z })
    }

    // 頂点列を契約の等高線へ写して空に戻す。頂点が1つも無いときは確定するものが無いためnullを返す。
    public 確定する(高さメートル: number, 閉じている: boolean): 等高線 | null {
        if (this._頂点列.length === 0) return null
        const 線: 等高線 = { 高さメートル, 頂点列: this._頂点列, 閉じている }
        this._頂点列 = []
        return 線
    }

    public 捨てる(): void {
        this._頂点列 = []
    }
}
