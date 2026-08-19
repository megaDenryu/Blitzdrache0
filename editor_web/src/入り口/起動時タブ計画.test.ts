import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 起動時タブ計画を立てる } from './起動時タブ計画.ts'

describe('起動時タブ計画', () => {
    it('使い方ガイド未閲覧なら大域世界の直後に使い方を積み、使い方が末尾(前面)になること', () => {
        assert.deepStrictEqual(起動時タブ計画を立てる(false), ['大域世界', '使い方'])
    })

    it('使い方ガイド閲覧済みなら大域世界だけを積むこと', () => {
        assert.deepStrictEqual(起動時タブ計画を立てる(true), ['大域世界'])
    })
})
