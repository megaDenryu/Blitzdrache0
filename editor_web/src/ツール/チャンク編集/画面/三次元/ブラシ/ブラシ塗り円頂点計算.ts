import type { 高さ場 } from '../../../編集モデル/index.ts'
import { ブラシ浮上量メートル } from './ブラシ角度列.ts'

// 塗り円の添字配列(中心から放射状に張った三角形扇)を作る。
export function ブラシ塗り円添字を作る(分割数: number): Uint16Array {
    const 添字配列 = new Uint16Array(分割数 * 3)
    let 位置 = 0
    for (let i = 0; i < 分割数; i++) {
        const 次 = (i + 1) % 分割数
        添字配列[位置++] = 0
        添字配列[位置++] = 1 + i
        添字配列[位置++] = 1 + 次
    }
    return 添字配列
}

// 塗り円の頂点位置バッファ(中心1点+周囲分割数点、xyzの順)を、中心位置・半径・地形の
// 高さ場から書き戻す。ブラシリング頂点計算.tsと同じくローカル座標(中心からの相対)で書く。
export function ブラシ塗り円頂点を更新する(
    頂点配列: Float32Array,
    角度列: Float64Array,
    中心X: number,
    中心Z: number,
    半径メートル: number,
    地形高さ場: 高さ場,
): void {
    頂点配列[0] = 0
    頂点配列[1] = 地形高さ場.標本高さを取得する(中心X, 中心Z) + ブラシ浮上量メートル
    頂点配列[2] = 0

    for (let i = 0; i < 角度列.length; i++) {
        const 角度 = 角度列[i] ?? 0
        const worldX = 中心X + Math.cos(角度) * 半径メートル
        const worldZ = 中心Z + Math.sin(角度) * 半径メートル
        const 添字 = (i + 1) * 3
        頂点配列[添字 + 0] = worldX - 中心X
        頂点配列[添字 + 1] = 地形高さ場.標本高さを取得する(worldX, worldZ) + ブラシ浮上量メートル
        頂点配列[添字 + 2] = worldZ - 中心Z
    }
}
