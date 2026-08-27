import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 音高番号の音名表記, トラック行の表示名 } from './音名表示.ts'

describe('音名表示の単体テスト', () => {
    it('音高番号から音名表記が正しく導出されること', () => {
        assert.strictEqual(音高番号の音名表記(60), 'C4')
        assert.strictEqual(音高番号の音名表記(69), 'A4')
        assert.strictEqual(音高番号の音名表記(70), 'A#4')
        assert.strictEqual(音高番号の音名表記(48), 'C3')
    })

    it('トラック行の表示名が旋律と打楽器で正しく返ること', () => {
        const 旋律並び = { 種類: '音高の行一覧' as const, 値: [60, 62, 64] }
        assert.strictEqual(トラック行の表示名(旋律並び, 0), 'C4')
        assert.strictEqual(トラック行の表示名(旋律並び, 1), 'D4')

        const 打楽器並び = { 種類: '打楽器の行一覧' as const, 値: ['バスドラム' as const, 'スネア' as const] }
        assert.strictEqual(トラック行の表示名(打楽器並び, 0), 'バスドラム')
        assert.strictEqual(トラック行の表示名(打楽器並び, 1), 'スネア')
    })
})
