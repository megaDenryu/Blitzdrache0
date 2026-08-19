import type { 中心線標本, 水平位置 } from './道路中心線標本列.ts'
import { 前進を保つ横距離を決める, 列の順に単調な横距離へ整える } from './道路帯オフセット横距離.ts'

// 帯の列ごとのオフセット点列。中心線を横へずらすだけでは急な曲がりの内側で点列が交差するため、
// 標本ごとに横距離を中心線側へ引き戻して交差を潰した結果を保持する。
// 引き戻しの単調性は行(標本)の中の列どうしを見て決めるため、列ごとに独立して作らず行を単位に作る。
export class 道路帯の列ごとの点列 {
    private constructor(private readonly _列ごとの点列: readonly (readonly 水平位置[])[]) {}

    // 中心線標本列と列ごとの横距離から、引き戻し込みでオフセット点列を組み立てる。
    public static 標本列から組み立てる(
        標本列: readonly 中心線標本[],
        列ごとの横距離: readonly number[],
    ): 道路帯の列ごとの点列 {
        const 列ごとの点列: 水平位置[][] = 列ごとの横距離.map(() => [])
        for (let 行 = 0; 行 < 標本列.length; 行++) {
            const 標本 = 標本列[行]
            if (標本 === undefined) continue
            const 横距離一覧 = 道路帯の列ごとの点列._一行の横距離一覧を決める(標本列, 行, 列ごとの横距離)
            列の順に単調な横距離へ整える(横距離一覧, 列ごとの横距離)
            for (let 列 = 0; 列 < 列ごとの点列.length; 列++) {
                const 横距離 = 横距離一覧[列] ?? 0
                列ごとの点列[列]?.push({
                    x: 標本.中心位置.x + 標本.横方向.x * 横距離,
                    z: 標本.中心位置.z + 標本.横方向.z * 横距離,
                })
            }
        }
        return new 道路帯の列ごとの点列(列ごとの点列)
    }

    public get 列ごとの点列(): readonly (readonly 水平位置[])[] {
        return this._列ごとの点列
    }

    private static _一行の横距離一覧を決める(
        標本列: readonly 中心線標本[],
        行: number,
        列ごとの横距離: readonly number[],
    ): number[] {
        const 標本 = 標本列[行]
        if (標本 === undefined) return [...列ごとの横距離]
        return 列ごとの横距離.map((元の横距離) =>
            前進を保つ横距離を決める(標本, 標本列[行 - 1], 標本列[行 + 1], 元の横距離),
        )
    }
}
