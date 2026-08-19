export interface キー移動方向 {
    readonly 前後: number
    readonly 左右: number
    readonly 上下: number
}

// WASDQEの押下状態から移動方向を求める。前後はW(前進)/S(後退)、左右はD(右)/A(左)、
// 上下はE(上昇)/Q(下降)。対となるキーの同時押しは相殺し、各成分は-1・0・1のいずれかになる。
export function キー状態から移動方向を決める(押下中キー集合: ReadonlySet<string>): キー移動方向 {
    const 前 = 押下中キー集合.has('w') ? 1 : 0
    const 後 = 押下中キー集合.has('s') ? 1 : 0
    const 右 = 押下中キー集合.has('d') ? 1 : 0
    const 左 = 押下中キー集合.has('a') ? 1 : 0
    const 上 = 押下中キー集合.has('e') ? 1 : 0
    const 下 = 押下中キー集合.has('q') ? 1 : 0
    return {
        前後: 前 - 後,
        左右: 右 - 左,
        上下: 上 - 下,
    }
}
