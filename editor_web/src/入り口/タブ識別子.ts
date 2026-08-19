import type { チャンク座標 } from '../境界/通信/index.ts'
import { 大域世界表示名 } from '../境界/index.ts'

export const 大域世界タブID = 大域世界表示名
export const 使い方タブID = '使い方'

// チャンク座標からタブの一意識別子を生成する。
export function チャンクタブIDを生成する(座標: チャンク座標): string {
    return `チャンク_${座標.x}_${座標.z}`
}

// タブ識別子がチャンクのものであれば座標を復元する。
export function タブIDからチャンク座標を復元する(タブID: string): チャンク座標 | null {
    if (!タブID.startsWith('チャンク_')) return null
    const 部分 = タブID.slice('チャンク_'.length).split('_')
    if (部分.length !== 2) return null
    const x = Number(部分[0])
    const z = Number(部分[1])
    if (Number.isNaN(x) || Number.isNaN(z)) return null
    return { x, z }
}
