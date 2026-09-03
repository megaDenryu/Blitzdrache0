import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 同じ粗マスか, 粗マスの格子 } from './粗マスの座標変換.ts'

// 256mのチャンク、2m刻みの格子(解像度129)、粗マスは8升目=16m。一辺に16個並ぶ。
const 格子 = 粗マスの格子.生成する(256, 2, 8)

describe('粗マスの座標変換', () => {
    it('粗マスの一辺は格子間隔と升目数の積になること', () => {
        assert.strictEqual(格子.粗マスの一辺のメートル(), 16)
        assert.strictEqual(格子.一辺に並ぶ粗マスの数(), 16)
    })

    it('北西の角の点は(0,0)の粗マスに入ること', () => {
        assert.deepStrictEqual(格子.ワールドから粗マスへ({ x: -128, z: -128 }), { 列: 0, 行: 0 })
    })

    it('チャンク中心のすぐ南東は(8,8)、すぐ北西は(7,7)になること', () => {
        assert.deepStrictEqual(格子.ワールドから粗マスへ({ x: 0.5, z: 0.5 }), { 列: 8, 行: 8 })
        assert.deepStrictEqual(格子.ワールドから粗マスへ({ x: -0.5, z: -0.5 }), { 列: 7, 行: 7 })
    })

    it('南東の端の点は最後の粗マス(15,15)に入ること', () => {
        assert.deepStrictEqual(格子.ワールドから粗マスへ({ x: 128, z: 128 }), { 列: 15, 行: 15 })
    })

    it('チャンクの外はnullになること', () => {
        assert.strictEqual(格子.ワールドから粗マスへ({ x: 129, z: 0 }), null)
        assert.strictEqual(格子.ワールドから粗マスへ({ x: 0, z: -128.1 }), null)
    })

    it('北西の角の座標が番地から戻ること', () => {
        assert.deepStrictEqual(格子.粗マスの北西の角({ 列: 8, 行: 0 }), { x: 0, z: -128 })
        const 角 = 格子.粗マスの北西の角({ 列: 3, 行: 5 })
        assert.deepStrictEqual(格子.ワールドから粗マスへ({ x: 角.x + 1, z: 角.z + 1 }), { 列: 3, 行: 5 })
    })

    it('同じ番地かの判定が列と行の両方を見ること', () => {
        assert.strictEqual(同じ粗マスか({ 列: 1, 行: 2 }, { 列: 1, 行: 2 }), true)
        assert.strictEqual(同じ粗マスか({ 列: 1, 行: 2 }, { 列: 2, 行: 1 }), false)
    })

    it('非正の寸法は生成で拒まれること', () => {
        assert.throws(() => 粗マスの格子.生成する(0, 2, 8), /一辺のメートル/)
        assert.throws(() => 粗マスの格子.生成する(256, 0, 8), /格子間隔/)
        assert.throws(() => 粗マスの格子.生成する(256, 2, 0), /粗マスの一辺の升目数/)
    })
})
