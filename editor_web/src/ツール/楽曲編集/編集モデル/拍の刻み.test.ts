import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 拍あたりのステップ数, 小節あたりのステップ数, 小節あたりの拍数 } from './演奏の時刻/index.ts'

describe('拍の刻み', () => {
    it('契約の小節あたりのステップ数が拍あたりのステップ数で割り切れ、1小節が4拍になる', () => {
        assert.ok(Number.isInteger(小節あたりの拍数), `小節あたりの拍数=${小節あたりの拍数}`)
        assert.equal(小節あたりの拍数, 4)
        assert.equal(拍あたりのステップ数 * 小節あたりの拍数, 小節あたりのステップ数)
    })
})
