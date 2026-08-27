// いま鳴っているのがどのパターンの何ステップ目かを表す。通しステップは演奏を始めてから数えた数である。
export interface 再生位置 {
    readonly パターンの名乗り: string
    readonly パターン内ステップ: number
    readonly 通しステップ: number
}

// 再生位置の変化を受け取る側の規約。停止したときは位置なし(null)が届く。
export interface I再生位置の届け先 {
    再生位置が変わった(位置: 再生位置 | null): void
}
