import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 道路点の挿入先を求める } from './道路点の挿入先.ts'

const 折れ線の制御点列 = [
    { x: 0, y: 0, z: 0 },
    { x: 100, y: 0, z: 0 },
    { x: 100, y: 0, z: 100 },
]

describe('道路点の挿入先', () => {
    it('いちばん近い区間の番号の次を添字として返すこと', () => {
        const 挿入先 = 道路点の挿入先を求める(折れ線の制御点列, { x: 100, y: 0, z: 40 })
        assert.notStrictEqual(挿入先, null)
        assert.strictEqual(挿入先?.添字, 2)
    })

    it('区間から外れた点でも区間の上へ落とした足を位置として返すこと', () => {
        const 挿入先 = 道路点の挿入先を求める(折れ線の制御点列, { x: 30, y: 12, z: 7 })
        assert.deepStrictEqual(挿入先, { 添字: 1, 位置: { x: 30, y: 0, z: 0 } })
    })

    it('区間の端より外側の点は端へ丸めること', () => {
        const 挿入先 = 道路点の挿入先を求める(
            [
                { x: 0, y: 0, z: 0 },
                { x: 10, y: 0, z: 0 },
            ],
            { x: -50, y: 0, z: 0 },
        )
        assert.deepStrictEqual(挿入先, { 添字: 1, 位置: { x: 0, y: 0, z: 0 } })
    })

    it('制御点が2つに満たないときはnullを返すこと', () => {
        assert.strictEqual(道路点の挿入先を求める([{ x: 0, y: 0, z: 0 }], { x: 1, y: 0, z: 1 }), null)
        assert.strictEqual(道路点の挿入先を求める([], { x: 1, y: 0, z: 1 }), null)
    })
})
