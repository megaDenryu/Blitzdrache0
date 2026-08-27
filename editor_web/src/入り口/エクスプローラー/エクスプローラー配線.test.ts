import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { チャンク座標, 建物定義ID, 楽曲ID } from '../../境界/通信/index.ts'
import { 建物定義IDを生成する, 楽曲IDを生成する } from '../../境界/index.ts'
import type { I世界エクスプローラー配線 } from './世界エクスプローラー.ts'
import type { I建物エクスプローラー配線 } from './建物エクスプローラー.ts'
import type { I楽曲エクスプローラー配線 } from './楽曲エクスプローラー.ts'

describe('編集領域ごとのエクスプローラー配線ポートの結合テスト', () => {
    it('世界の配線が大域世界・チャンク・マテリアルのオープン要求だけを受け取ること', () => {
        let 大域世界開いた = false
        let マテリアル開いた = false
        const 開いたチャンク座標: チャンク座標[] = []

        const 配線: I世界エクスプローラー配線 = {
            on大域世界を開く: () => { 大域世界開いた = true },
            onチャンクを開く: (座標) => { 開いたチャンク座標.push(座標) },
            onマテリアルを開く: () => { マテリアル開いた = true },
        }

        配線.on大域世界を開く()
        配線.onチャンクを開く({ x: 1, z: 2 })
        配線.onマテリアルを開く()

        assert.strictEqual(大域世界開いた, true)
        assert.strictEqual(マテリアル開いた, true)
        assert.strictEqual(開いたチャンク座標.length, 1)
        assert.strictEqual(開いたチャンク座標[0]?.x, 1)
        assert.strictEqual(開いたチャンク座標[0]?.z, 2)
    })

    it('建物の配線が建物を開く要求と建物を作る要求を受け取ること', () => {
        let 建物作った = false
        const 開いた建物: { 建物定義ID: 建物定義ID; 表示名: string }[] = []

        const 配線: I建物エクスプローラー配線 = {
            on建物を開く: (建物定義ID, 表示名) => { 開いた建物.push({ 建物定義ID, 表示名 }) },
            on建物を作る: () => { 建物作った = true },
        }

        配線.on建物を開く(建物定義IDを生成する('b-1'), '建物1')
        配線.on建物を作る()

        assert.strictEqual(開いた建物.length, 1)
        assert.strictEqual(開いた建物[0]?.建物定義ID, 建物定義IDを生成する('b-1'))
        assert.strictEqual(建物作った, true)
    })

    it('楽曲の配線が楽曲を開く要求と楽曲を作る要求を受け取ること', () => {
        let 楽曲作った = false
        const 開いた楽曲: { 楽曲ID: 楽曲ID; 表示名: string }[] = []

        const 配線: I楽曲エクスプローラー配線 = {
            on楽曲を開く: (楽曲ID, 表示名) => { 開いた楽曲.push({ 楽曲ID, 表示名 }) },
            on楽曲を作る: () => { 楽曲作った = true },
        }

        配線.on楽曲を開く(楽曲IDを生成する('song-1'), '楽曲1')
        配線.on楽曲を作る()

        assert.strictEqual(開いた楽曲.length, 1)
        assert.strictEqual(開いた楽曲[0]?.楽曲ID, 楽曲IDを生成する('song-1'))
        assert.strictEqual(楽曲作った, true)
    })
})
