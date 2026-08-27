import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { チャンク座標, 建物定義ID, 楽曲ID } from '../../境界/通信/index.ts'
import { 建物定義IDを生成する, 楽曲IDを生成する } from '../../境界/index.ts'
import type { Iエクスプローラー配線 } from './エクスプローラーパネル.ts'

describe('エクスプローラー配線ポートの結合テスト', () => {
    it('配線インターフェースを通じて大域世界・チャンク・マテリアル・建物・楽曲・使い方のオープン要求を受け取れること', () => {
        let 大域世界開いた = false
        let マテリアル開いた = false
        let 使い方開いた = false
        let 建物作った = false
        let 楽曲作った = false
        const 開いたチャンク座標: チャンク座標[] = []
        const 開いた建物: { 建物定義ID: 建物定義ID; 表示名: string }[] = []
        const 開いた楽曲: { 楽曲ID: 楽曲ID; 表示名: string }[] = []

        const 配線: Iエクスプローラー配線 = {
            on大域世界を開く: () => { 大域世界開いた = true },
            onチャンクを開く: (座標) => { 開いたチャンク座標.push(座標) },
            onマテリアルを開く: () => { マテリアル開いた = true },
            on建物を開く: (建物定義ID, 表示名) => { 開いた建物.push({ 建物定義ID, 表示名 }) },
            on建物を作る: () => { 建物作った = true },
            on楽曲を開く: (楽曲ID, 表示名) => { 開いた楽曲.push({ 楽曲ID, 表示名 }) },
            on楽曲を作る: () => { 楽曲作った = true },
            on使い方を開く: () => { 使い方開いた = true },
        }

        配線.on大域世界を開く()
        配線.onチャンクを開く({ x: 1, z: 2 })
        配線.onマテリアルを開く()
        配線.on建物を開く(建物定義IDを生成する('b-1'), '建物1')
        配線.on建物を作る()
        配線.on楽曲を開く(楽曲IDを生成する('song-1'), '楽曲1')
        配線.on楽曲を作る()
        配線.on使い方を開く()

        assert.strictEqual(大域世界開いた, true)
        assert.strictEqual(開いたチャンク座標.length, 1)
        assert.strictEqual(開いたチャンク座標[0]?.x, 1)
        assert.strictEqual(開いたチャンク座標[0]?.z, 2)
        assert.strictEqual(マテリアル開いた, true)
        assert.strictEqual(開いた建物.length, 1)
        assert.strictEqual(建物作った, true)
        assert.strictEqual(開いた楽曲.length, 1)
        assert.strictEqual(開いた楽曲[0]?.楽曲ID, 楽曲IDを生成する('song-1'))
        assert.strictEqual(楽曲作った, true)
        assert.strictEqual(使い方開いた, true)
    })
})

