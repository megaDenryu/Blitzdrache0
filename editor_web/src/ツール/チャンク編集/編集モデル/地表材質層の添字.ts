import type { 地表材質層 } from '../../../生成/編集資源契約.ts'

// 材質データの1画素は草・泥・岩・砂の順で4バイトを持つ。この並びは材質重み.u8rawの契約であり、変えない。
export const 地表材質層の一覧: readonly 地表材質層[] = ['草', '泥', '岩', '砂']
export const 地表材質層の数 = 4

export function 地表材質層を添字に変換する(層: 地表材質層): number {
    switch (層) {
        case '草': return 0
        case '泥': return 1
        case '岩': return 2
        case '砂': return 3
    }
    const 未知の層: never = 層
    throw new Error(`未知の地表材質層: ${JSON.stringify(未知の層)}`)
}

export function 添字を地表材質層に変換する(添字: number): 地表材質層 {
    const 層 = 地表材質層の一覧[添字]
    if (層 === undefined) {
        throw new Error(`地表材質層の添字が範囲外: ${添字}`)
    }
    return 層
}
