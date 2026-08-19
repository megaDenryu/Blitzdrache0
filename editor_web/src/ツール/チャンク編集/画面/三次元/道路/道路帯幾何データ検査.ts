import type { 道路帯幾何データ } from './道路帯頂点配列組み立て.ts'

// 帯の三角形は「行方向=進行、列方向=左から右」の順で張るため、水平面へ落とした符号付き面積は
// 裏返っていない限り常に0以上になる。急な曲がりの内側で頂点が重なった箇所だけが面積0になる。
// 帯の生成が壊れていないことを機械で確かめるための検査であり、テストから呼ぶ。
export function 裏返った三角形の枚数を数える(幾何: 道路帯幾何データ, 許容面積: number): number {
    let 枚数 = 0
    for (let 位置 = 0; 位置 + 2 < 幾何.添字配列.length; 位置 += 3) {
        if (符号付き面積を求める(幾何, 位置) < -許容面積) 枚数++
    }
    return 枚数
}

export function 面積を持つ三角形があるか(幾何: 道路帯幾何データ, 許容面積: number): boolean {
    for (let 位置 = 0; 位置 + 2 < 幾何.添字配列.length; 位置 += 3) {
        if (符号付き面積を求める(幾何, 位置) > 許容面積) return true
    }
    return false
}

export function 非数を含まないか(幾何: 道路帯幾何データ): boolean {
    return 幾何.頂点配列.every((値) => Number.isFinite(値)) && 幾何.UV配列.every((値) => Number.isFinite(値))
}

function 水平位置を読む(幾何: 道路帯幾何データ, 添字位置: number): { x: number; z: number } {
    const 添字 = 幾何.添字配列[添字位置] ?? 0
    return { x: 幾何.頂点配列[添字 * 3] ?? 0, z: 幾何.頂点配列[添字 * 3 + 2] ?? 0 }
}

function 符号付き面積を求める(幾何: 道路帯幾何データ, 添字位置: number): number {
    const 頂点1 = 水平位置を読む(幾何, 添字位置)
    const 頂点2 = 水平位置を読む(幾何, 添字位置 + 1)
    const 頂点3 = 水平位置を読む(幾何, 添字位置 + 2)
    return 0.5 * ((頂点2.x - 頂点1.x) * (頂点3.z - 頂点1.z) - (頂点2.z - 頂点1.z) * (頂点3.x - 頂点1.x))
}
