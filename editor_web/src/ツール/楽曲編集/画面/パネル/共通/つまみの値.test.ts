import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { つまみの文字列を数値として読む } from './つまみの値.ts'

describe('つまみの値', () => {
    it('つまみの文字列を数値として読めること', () => {
        assert.strictEqual(つまみの文字列を数値として読む('110'), 110)
        assert.strictEqual(つまみの文字列を数値として読む('0.35'), 0.35)
        assert.strictEqual(つまみの文字列を数値として読む('0'), 0)
    })

    it('読めない文字列を既定値へ落とさず、明示の失敗にすること', () => {
        assert.throws(() => つまみの文字列を数値として読む(''))
        assert.throws(() => つまみの文字列を数値として読む('とても大きい'))
    })
})
