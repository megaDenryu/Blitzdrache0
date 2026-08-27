import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 音を出せなかった知らせを組み立てる } from './演奏の知らせ.ts'

describe('演奏の知らせ', () => {
    it('音を出せなかったとき、利用者が次に何をすればよいかが文言に書かれていること', () => {
        const 知らせ = 音を出せなかった知らせを組み立てる(new Error('NotAllowedError'))
        assert.strictEqual(知らせ.種類, '音を出せなかった')
        assert.match(知らせ.文言, /もう一度/)
        assert.match(知らせ.文言, /再生/)
        assert.ok(知らせ.文言.length > 0)
    })

    it('もとの失敗の理由が、詳しい原因として残ること', () => {
        assert.strictEqual(音を出せなかった知らせを組み立てる(new Error('拒まれた')).詳しい原因, '拒まれた')
        assert.strictEqual(音を出せなかった知らせを組み立てる('文字列の失敗').詳しい原因, '文字列の失敗')
        assert.strictEqual(音を出せなかった知らせを組み立てる(null).詳しい原因, 'null')
    })
})
