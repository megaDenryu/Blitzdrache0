import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 大域世界タブID, チャンクタブIDを生成する, タブIDからチャンク座標を復元する } from './タブ識別子.ts'

describe('タブ識別子の相互変換テスト', () => {
    it('チャンク座標からタブIDを生成し正しく座標を復元できること', () => {
        const 座標 = { x: 2, z: 3 }
        const タブID = チャンクタブIDを生成する(座標)
        assert.strictEqual(タブID, 'チャンク_2_3')

        const 復元 = タブIDからチャンク座標を復元する(タブID)
        assert.ok(復元 !== null)
        assert.strictEqual(復元.x, 2)
        assert.strictEqual(復元.z, 3)
    })

    it('負の座標を含むチャンク座標でも往復できること', () => {
        const 座標 = { x: -1, z: -4 }
        const タブID = チャンクタブIDを生成する(座標)
        assert.strictEqual(タブID, 'チャンク_-1_-4')

        const 復元 = タブIDからチャンク座標を復元する(タブID)
        assert.ok(復元 !== null)
        assert.strictEqual(復元.x, -1)
        assert.strictEqual(復元.z, -4)
    })

    it('大域世界タブIDや不正なタブIDではnullを返すこと', () => {
        assert.strictEqual(タブIDからチャンク座標を復元する(大域世界タブID), null)
        assert.strictEqual(タブIDからチャンク座標を復元する('unknown_tab'), null)
        assert.strictEqual(タブIDからチャンク座標を復元する('チャンク_abc_def'), null)
    })
})
