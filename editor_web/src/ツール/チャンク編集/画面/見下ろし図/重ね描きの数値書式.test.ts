import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 高さを図の文字にする } from './重ね描きの数値書式.ts'

describe('高さを図の文字にする', () => {
    it('小数第1位で丸めること', () => {
        assert.strictEqual(高さを図の文字にする(12.46), '12.5')
    })

    it('丸めた結果が0になる負の値を0.0にすること(マイナスゼロにしない)', () => {
        assert.strictEqual(高さを図の文字にする(-0.04), '0.0')
    })

    it('絶対値が100以上なら整数にすること', () => {
        assert.strictEqual(高さを図の文字にする(128.7), '129')
    })

    it('絶対値が100以上の整数値はそのまま整数の文字にすること', () => {
        assert.strictEqual(高さを図の文字にする(1000), '1000')
    })
})
