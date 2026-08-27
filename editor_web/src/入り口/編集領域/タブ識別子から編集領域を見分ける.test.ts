import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 建物定義IDを生成する, 楽曲IDを生成する } from '../../境界/index.ts'
import { タブ識別子 } from '../タブ識別子.ts'
import { タブ識別子から編集領域を見分ける } from './タブ識別子から編集領域を見分ける.ts'
import { 世界の編集領域, 建物の編集領域, 楽曲の編集領域 } from './編集領域の名乗り.ts'

describe('タブがどの編集領域に属するかの見分け', () => {
    it('大域世界・チャンク・マテリアルのタブが世界の領域に属すること', () => {
        assert.strictEqual(タブ識別子から編集領域を見分ける(タブ識別子.大域世界()), 世界の編集領域)
        assert.strictEqual(タブ識別子から編集領域を見分ける(タブ識別子.マテリアル()), 世界の編集領域)
        assert.strictEqual(タブ識別子から編集領域を見分ける(タブ識別子.チャンクから生成する({ x: 1, z: 2 })), 世界の編集領域)
    })

    it('建物のタブが建物の領域に属すること', () => {
        const タブ = タブ識別子.建物から生成する(建物定義IDを生成する('grid_building_1'))
        assert.strictEqual(タブ識別子から編集領域を見分ける(タブ), 建物の編集領域)
    })

    it('楽曲のタブが楽曲の領域に属すること', () => {
        const タブ = タブ識別子.楽曲から生成する(楽曲IDを生成する('song-1'))
        assert.strictEqual(タブ識別子から編集領域を見分ける(タブ), 楽曲の編集領域)
    })

    it('使い方のタブはどの編集領域にも属さないこと', () => {
        assert.strictEqual(タブ識別子から編集領域を見分ける(タブ識別子.使い方()), null)
    })

    it('見覚えのない綴りのタブはどの編集領域にも属さないこと', () => {
        assert.strictEqual(タブ識別子から編集領域を見分ける(タブ識別子.綴りから復元する('unknown_tab')), null)
    })
})
