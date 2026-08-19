import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { タブ識別子 } from './タブ識別子.ts'

describe('タブ識別子の相互変換テスト', () => {
    it('チャンク座標からタブ識別子を生成し正しく座標を復元できること', () => {
        const 座標 = { x: 2, z: 3 }
        const 識別子 = タブ識別子.チャンクから生成する(座標)
        assert.strictEqual(識別子.綴り(), 'チャンク_2_3')

        const 復元 = 識別子.チャンク座標を復元する()
        assert.ok(復元 !== null)
        assert.strictEqual(復元.x, 2)
        assert.strictEqual(復元.z, 3)
    })

    it('負の座標を含むチャンク座標でも往復できること', () => {
        const 座標 = { x: -1, z: -4 }
        const 識別子 = タブ識別子.チャンクから生成する(座標)
        assert.strictEqual(識別子.綴り(), 'チャンク_-1_-4')

        const 復元 = 識別子.チャンク座標を復元する()
        assert.ok(復元 !== null)
        assert.strictEqual(復元.x, -1)
        assert.strictEqual(復元.z, -4)
    })

    it('大域世界タブ識別子や不正な綴りではnullを返すこと', () => {
        assert.strictEqual(タブ識別子.大域世界().チャンク座標を復元する(), null)
        assert.strictEqual(タブ識別子.綴りから復元する('unknown_tab').チャンク座標を復元する(), null)
        assert.strictEqual(タブ識別子.綴りから復元する('チャンク_abc_def').チャンク座標を復元する(), null)
    })
})
