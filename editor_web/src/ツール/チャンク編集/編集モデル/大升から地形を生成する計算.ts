import type { 大升の塗り } from '../../../生成/編集資源契約.ts'
import { 大升の分割を求める, 大升の添字を求める, 大升ごとの升内平均を求める, type 大升の分割 } from './大升の格子分割.ts'
import { 地表材質層の数, 地表材質層を添字に変換する } from './地表材質層の添字.ts'
import { 合計255へ正規化する } from './地表材質正規化.ts'

// 大升の塗りから高さ格子と材質重み格子を作り直す純粋計算。大升の中心を制御点にし、塗られた大升は塗りの
// 高さと層(その層だけ255の重み)を、塗られていない大升は現在の升内平均を制御点の値にして、双線形補間で
// 全格子点へ写す。周囲4つの制御点がどれも塗られていない格子点は、現在の値をそのまま残す(塗っていない
// 大升の中を変えないため。平均で補間し直すとその場所の起伏が消えてしまう)。外周の格子点は変えない。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断4」。

export type 大升から生成した地形 = {
    readonly 高さ: Float32Array
    readonly 材質: Uint8Array
}

type 制御点の値 = {
    readonly 塗られている: Uint8Array
    readonly 高さ: Float64Array
    readonly 重み: Float64Array
}

export function 大升から地形を生成する(
    塗り一覧: ReadonlyArray<大升の塗り>,
    一辺の升目数: number,
    解像度: number,
    現在の格子データ: Float32Array,
    現在の材質データ: Uint8Array,
): 大升から生成した地形 {
    const 分割 = 大升の分割を求める(解像度, 一辺の升目数)
    const 制御点 = 制御点の値を組む(分割, 塗り一覧, 解像度, 現在の格子データ, 現在の材質データ)
    const 高さ = new Float32Array(現在の格子データ)
    const 材質 = new Uint8Array(現在の材質データ)
    const 最後の大升 = 分割.軸あたり大升数 - 1
    for (let gz = 1; gz < 解像度 - 1; gz++) {
        for (let gx = 1; gx < 解像度 - 1; gx++) {
            const u = Math.min(最後の大升, Math.max(0, gx / 分割.一辺の升目数 - 0.5))
            const v = Math.min(最後の大升, Math.max(0, gz / 分割.一辺の升目数 - 0.5))
            const 列0 = Math.min(最後の大升, Math.floor(u))
            const 行0 = Math.min(最後の大升, Math.floor(v))
            const 列1 = Math.min(最後の大升, 列0 + 1)
            const 行1 = Math.min(最後の大升, 行0 + 1)
            const 角 = [行0 * 分割.軸あたり大升数 + 列0, 行0 * 分割.軸あたり大升数 + 列1, 行1 * 分割.軸あたり大升数 + 列0, 行1 * 分割.軸あたり大升数 + 列1]
            if (角.every((c) => 制御点.塗られている[c] === 0)) continue
            const fx = u - 列0
            const fz = v - 行0
            const 係数 = [(1 - fx) * (1 - fz), fx * (1 - fz), (1 - fx) * fz, fx * fz]
            const 格子添字 = gz * 解像度 + gx
            高さ[格子添字] = 補間する(制御点.高さ, 角, 係数, 1, 0)
            const 重み = [0, 1, 2, 3].map((層) => 補間する(制御点.重み, 角, 係数, 地表材質層の数, 層))
            const 正規化 = 合計255へ正規化する(重み[0] ?? 0, 重み[1] ?? 0, 重み[2] ?? 0, 重み[3] ?? 0)
            for (let 層 = 0; 層 < 地表材質層の数; 層++) {
                材質[格子添字 * 地表材質層の数 + 層] = 正規化[層] ?? 0
            }
        }
    }
    return { 高さ, 材質 }
}

function 補間する(値: Float64Array, 角: number[], 係数: number[], 幅: number, 成分: number): number {
    let 合計 = 0
    for (let i = 0; i < 4; i++) {
        合計 += (値[(角[i] ?? 0) * 幅 + 成分] ?? 0) * (係数[i] ?? 0)
    }
    return 合計
}

// 高さがnullの大升は高さだけ現状平均、層がnullの大升は重みだけ現状平均を使う。
function 制御点の値を組む(
    分割: 大升の分割,
    塗り一覧: ReadonlyArray<大升の塗り>,
    解像度: number,
    格子データ: Float32Array,
    材質データ: Uint8Array,
): 制御点の値 {
    const 平均 = 大升ごとの升内平均を求める(分割, 解像度, 格子データ, 材質データ)
    const 塗られている = new Uint8Array(分割.軸あたり大升数 * 分割.軸あたり大升数)
    const 高さ = new Float64Array(平均.高さ平均)
    const 重み = new Float64Array(平均.重み平均)
    for (const 塗り of 塗り一覧) {
        const 大升添字 = 大升の添字を求める(分割, 塗り.列, 塗り.行)
        塗られている[大升添字] = 1
        if (塗り.高さメートル !== null) 高さ[大升添字] = 塗り.高さメートル
        if (塗り.層 !== null) {
            重み.fill(0, 大升添字 * 地表材質層の数, (大升添字 + 1) * 地表材質層の数)
            重み[大升添字 * 地表材質層の数 + 地表材質層を添字に変換する(塗り.層)] = 255
        }
    }
    return { 塗られている, 高さ, 重み }
}
