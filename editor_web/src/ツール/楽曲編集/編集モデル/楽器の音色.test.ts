import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { 打楽器の種類 } from '../../../生成/編集資源契約.ts'
import { 楽器の音色を求める, 音色を持つ楽器一覧 } from './楽器の音色/index.ts'

const 打楽器の全種類: readonly 打楽器の種類[] = ['バスドラム', 'スネア', 'ハイハット']

describe('楽器の音色', () => {
    it('契約の楽器11種すべてについて音色が引けること', () => {
        assert.strictEqual(音色を持つ楽器一覧.length, 11)
        for (const 対象の楽器 of 音色を持つ楽器一覧) {
            assert.doesNotThrow(() => 楽器の音色を求める(対象の楽器), `${対象の楽器} の音色が引けない`)
        }
    })

    it('打楽器の楽器は打楽器3種すべての作り方を持つこと', () => {
        for (const 対象の楽器 of ['生ドラム', '矩形波と雑音のドラム'] as const) {
            const 音色 = 楽器の音色を求める(対象の楽器)
            assert.strictEqual(音色.種類, '打楽器を鳴らす')
            if (音色.種類 !== '打楽器を鳴らす') return
            for (const 打楽器 of 打楽器の全種類) {
                assert.ok(音色.打楽器ごとの作り方[打楽器] !== undefined, `${対象の楽器}/${打楽器}`)
            }
        }
    })

    it('音高を鳴らす楽器の値が、音源が受け取れる範囲に収まっていること', () => {
        for (const 対象の楽器 of 音色を持つ楽器一覧) {
            const 音色 = 楽器の音色を求める(対象の楽器)
            if (音色.種類 !== '音高を鳴らす') continue
            const 作り方 = 音色.作り方
            switch (作り方.種類) {
                case '加算合成':
                    assert.ok(作り方.倍音一覧.length > 0, 対象の楽器)
                    for (const 倍音 of 作り方.倍音一覧) {
                        assert.ok(倍音.比 > 0 && 倍音.利得 > 0 && 倍音.利得 <= 1, 対象の楽器)
                        assert.ok(倍音.音を止めるまでの秒数 > 0, 対象の楽器)
                    }
                    break
                case '減算合成':
                    assert.ok(作り方.濾波の始まりの遮断周波数 > 0 && 作り方.濾波の終わりの遮断周波数 > 0, 対象の楽器)
                    assert.ok(作り方.立ち上がりの秒数 >= 0 && 作り方.減衰の秒数 >= 0, 対象の楽器)
                    break
                case '撥弦合成':
                    assert.ok(作り方.減衰の強さ > 0 && 作り方.減衰の強さ <= 1, 対象の楽器)
                    assert.ok(作り方.胴の濾波の開始遮断周波数 > 0 && 作り方.胴の濾波の終了遮断周波数 > 0, 対象の楽器)
                    break
                case '周波数変調合成':
                    assert.ok(作り方.変調波の周波数比 > 0 && 作り方.変調の深さ >= 0, 対象の楽器)
                    assert.ok(作り方.深さの減衰の秒数 > 0, 対象の楽器)
                    break
            }
        }
    })
})
