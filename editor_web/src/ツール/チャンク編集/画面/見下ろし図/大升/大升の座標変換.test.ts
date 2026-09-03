import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { ワールドから大升へ, 一辺に並ぶ大升の数, 同じ大升か, 大升の一辺のメートル, 大升の北西の角, type 大升の格子 } from './大升の座標変換.ts'

// 256mのチャンク、2m刻みの格子(解像度129)、大升は8升目=16m。一辺に16個並ぶ。
const 格子: 大升の格子 = { 一辺のメートル: 256, 格子間隔: 2, 大升の一辺の升目数: 8 }

describe('大升の座標変換', () => {
    it('大升の一辺は格子間隔と升目数の積になること', () => {
        assert.strictEqual(大升の一辺のメートル(格子), 16)
        assert.strictEqual(一辺に並ぶ大升の数(格子), 16)
    })

    it('北西の角の点は(0,0)の大升に入ること', () => {
        assert.deepStrictEqual(ワールドから大升へ({ x: -128, z: -128 }, 格子), { 列: 0, 行: 0 })
    })

    it('チャンク中心のすぐ南東は(8,8)、すぐ北西は(7,7)になること', () => {
        assert.deepStrictEqual(ワールドから大升へ({ x: 0.5, z: 0.5 }, 格子), { 列: 8, 行: 8 })
        assert.deepStrictEqual(ワールドから大升へ({ x: -0.5, z: -0.5 }, 格子), { 列: 7, 行: 7 })
    })

    it('南東の端の点は最後の大升(15,15)に入ること', () => {
        assert.deepStrictEqual(ワールドから大升へ({ x: 128, z: 128 }, 格子), { 列: 15, 行: 15 })
    })

    it('チャンクの外はnullになること', () => {
        assert.strictEqual(ワールドから大升へ({ x: 129, z: 0 }, 格子), null)
        assert.strictEqual(ワールドから大升へ({ x: 0, z: -128.1 }, 格子), null)
    })

    it('北西の角の座標が番地から戻ること', () => {
        assert.deepStrictEqual(大升の北西の角({ 列: 8, 行: 0 }, 格子), { x: 0, z: -128 })
        const 角 = 大升の北西の角({ 列: 3, 行: 5 }, 格子)
        assert.deepStrictEqual(ワールドから大升へ({ x: 角.x + 1, z: 角.z + 1 }, 格子), { 列: 3, 行: 5 })
    })

    it('同じ番地かの判定が列と行の両方を見ること', () => {
        assert.strictEqual(同じ大升か({ 列: 1, 行: 2 }, { 列: 1, 行: 2 }), true)
        assert.strictEqual(同じ大升か({ 列: 1, 行: 2 }, { 列: 2, 行: 1 }), false)
    })
})
