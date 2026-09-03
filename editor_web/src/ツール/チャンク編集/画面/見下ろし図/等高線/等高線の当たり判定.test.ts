import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { 等高線 } from '../../../../../生成/編集資源契約.ts'
import { 始点で閉じるか, 点と線分の距離, 線分の列を作る, 線分の当たりを探す, 頂点の当たりを探す } from './等高線の当たり判定.ts'

const 開いた線: 等高線 = { 高さメートル: 10, 頂点列: [{ x: 0, z: 0 }, { x: 10, z: 0 }, { x: 10, z: 10 }], 閉じている: false }
const 閉じた線: 等高線 = { 高さメートル: 20, 頂点列: [{ x: 50, z: 50 }, { x: 60, z: 50 }, { x: 60, z: 60 }], 閉じている: true }
const 一覧 = [開いた線, 閉じた線]

describe('点と線分の距離', () => {
    it('線分の途中への垂線の長さになること', () => {
        assert.strictEqual(点と線分の距離({ x: 5, z: 3 }, { x: 0, z: 0 }, { x: 10, z: 0 }), 3)
    })

    it('線分の外側では近い端点への距離になること', () => {
        assert.strictEqual(点と線分の距離({ x: 14, z: 3 }, { x: 0, z: 0 }, { x: 10, z: 0 }), 5)
    })

    it('線分が1点に潰れていてもその点への距離を返すこと', () => {
        assert.strictEqual(点と線分の距離({ x: 3, z: 4 }, { x: 0, z: 0 }, { x: 0, z: 0 }), 5)
    })
})

describe('頂点の当たりを探す', () => {
    it('半径の中で最も近い頂点を返すこと', () => {
        assert.deepStrictEqual(頂点の当たりを探す(一覧, { x: 60.5, z: 59.5 }, 1), { 線の添字: 1, 頂点の添字: 2 })
    })

    it('半径の外なら当たらないこと', () => {
        assert.strictEqual(頂点の当たりを探す(一覧, { x: 30, z: 30 }, 1), null)
    })
})

describe('線分の列を作る', () => {
    it('開いた線は頂点の数より1つ少ない線分を持つこと', () => {
        assert.strictEqual(線分の列を作る(開いた線).length, 2)
    })

    it('閉じた線は末尾から先頭へ戻る線分を含むこと', () => {
        const 列 = 線分の列を作る(閉じた線)
        assert.strictEqual(列.length, 3)
        assert.deepStrictEqual(列[2], [{ x: 60, z: 60 }, { x: 50, z: 50 }])
    })

    it('頂点1つの線は線分を持たないこと', () => {
        assert.strictEqual(線分の列を作る({ 高さメートル: 0, 頂点列: [{ x: 1, z: 1 }], 閉じている: true }).length, 0)
    })
})

describe('線分の当たりを探す', () => {
    it('線分の近くをクリックするとその線の添字を返すこと', () => {
        assert.strictEqual(線分の当たりを探す(一覧, { x: 5, z: 0.5 }, 1), 0)
    })

    it('閉じた線の戻りの線分にも当たること', () => {
        assert.strictEqual(線分の当たりを探す(一覧, { x: 55, z: 55.5 }, 1), 1)
    })

    it('どの線分からも遠いときは当たらないこと', () => {
        assert.strictEqual(線分の当たりを探す(一覧, { x: 30, z: 30 }, 1), null)
    })
})

describe('始点で閉じるか', () => {
    it('頂点が3つ以上あり始点の近くなら閉じること', () => {
        assert.strictEqual(始点で閉じるか(開いた線.頂点列, { x: 0.3, z: 0.3 }, 1), true)
    })

    it('頂点が2つでは始点の上でも閉じないこと', () => {
        assert.strictEqual(始点で閉じるか([{ x: 0, z: 0 }, { x: 5, z: 0 }], { x: 0, z: 0 }, 1), false)
    })

    it('始点から離れていれば閉じないこと', () => {
        assert.strictEqual(始点で閉じるか(開いた線.頂点列, { x: 5, z: 5 }, 1), false)
    })
})
