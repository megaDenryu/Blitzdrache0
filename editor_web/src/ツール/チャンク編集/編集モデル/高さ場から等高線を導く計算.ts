import type { 等高線 } from '../../../生成/編集資源契約.ts'
import { 等値線の線分を抽出する } from './等値線の線分抽出.ts'
import { 線分を折れ線へ連結する } from './等値線の線分の連結.ts'
import { 折れ線の頂点を間引く } from './折れ線の頂点の間引き.ts'

// 高さ格子から一定間隔の等高線一覧を導く純粋計算。間隔の整数倍の各高さについて、等値線の線分抽出・
// 線分の連結・頂点の間引きの3工程を順に通す。導いた等高線から生成し直しても元の高さ場は戻らない
// (間隔より細かい起伏は落ちる)ことを前提にする。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断5」。

export const 等高線の頂点の間引きの許容メートル = 0.5

export function 高さ場から等高線を導く(
    間隔メートル: number,
    解像度: number,
    一辺のメートル: number,
    格子データ: Float32Array,
): Array<等高線> {
    if (!(間隔メートル > 0)) {
        throw new Error(`等高線の間隔は正の数でなければならない: ${間隔メートル}`)
    }
    let 最小 = Number.POSITIVE_INFINITY
    let 最大 = Number.NEGATIVE_INFINITY
    for (const 値 of 格子データ) {
        if (値 < 最小) 最小 = 値
        if (値 > 最大) 最大 = 値
    }
    const 等高線一覧: Array<等高線> = []
    for (let k = Math.ceil(最小 / 間隔メートル); k <= Math.floor(最大 / 間隔メートル); k++) {
        const 高さ = k * 間隔メートル
        const 線分一覧 = 等値線の線分を抽出する(格子データ, 解像度, 一辺のメートル, 高さ)
        for (const 折れ線 of 線分を折れ線へ連結する(線分一覧)) {
            const 頂点列 = 折れ線の頂点を間引く(折れ線.頂点列, 等高線の頂点の間引きの許容メートル)
            if (頂点列.length < 2) continue
            等高線一覧.push({ 高さメートル: 高さ, 頂点列, 閉じている: 折れ線.閉じている && 頂点列.length >= 3 })
        }
    }
    return 等高線一覧
}
