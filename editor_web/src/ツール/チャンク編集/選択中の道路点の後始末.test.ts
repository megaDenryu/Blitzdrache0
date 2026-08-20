import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 道路の一覧 } from './編集モデル/index.ts'
import type { 道路点の在り処 } from './編集モデル/index.ts'
import { 居なくなった道路と道路点の選択を外す } from './選択中の道路点の後始末.ts'

function 制御点の数だけ並べた道路一覧(制御点の数一覧: readonly number[]): 道路の一覧 {
    return new 道路の一覧(
        制御点の数一覧.map((制御点の数) => ({
            制御点列: Array.from({ length: 制御点の数 }, (_値, 番号) => ({ x: 番号, y: 0, z: 0 })),
            全幅メートル: 8,
            散布除外バッファメートル: 12,
            細分割数: 40,
        })),
    )
}

function 選択状態を作る(選択中の道路点: 道路点の在り処 | null, アクティブな道路の添字: number | null) {
    return { 選択中の道路点, つかんでいる道路点: null as 道路点の在り処 | null, アクティブな道路の添字 }
}

describe('選択中の道路点の後始末', () => {
    it('制御点の数より大きい制御点添字を選んだままなら選択を外すこと', () => {
        const 状態 = 選択状態を作る({ 道路添字: 0, 制御点添字: 3 }, 0)
        居なくなった道路と道路点の選択を外す(状態, 制御点の数だけ並べた道路一覧([3]))
        assert.strictEqual(状態.選択中の道路点, null)
    })

    it('道そのものが消えていたら選択もアクティブな道も外すこと', () => {
        const 状態 = 選択状態を作る({ 道路添字: 2, 制御点添字: 0 }, 2)
        居なくなった道路と道路点の選択を外す(状態, 制御点の数だけ並べた道路一覧([3, 3]))
        assert.strictEqual(状態.選択中の道路点, null)
        assert.strictEqual(状態.アクティブな道路の添字, null)
    })

    it('まだ居る道路点を選んでいるなら選択もアクティブな道も保つこと', () => {
        const 状態 = 選択状態を作る({ 道路添字: 1, 制御点添字: 2 }, 1)
        居なくなった道路と道路点の選択を外す(状態, 制御点の数だけ並べた道路一覧([3, 3]))
        assert.deepStrictEqual(状態.選択中の道路点, { 道路添字: 1, 制御点添字: 2 })
        assert.strictEqual(状態.アクティブな道路の添字, 1)
    })

    it('何も選んでおらずアクティブな道も無いときは何もしないこと', () => {
        const 状態 = 選択状態を作る(null, null)
        居なくなった道路と道路点の選択を外す(状態, 制御点の数だけ並べた道路一覧([]))
        assert.strictEqual(状態.選択中の道路点, null)
        assert.strictEqual(状態.アクティブな道路の添字, null)
    })
})
