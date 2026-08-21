import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { チャンク座標 } from '../../境界/通信/index.ts'
import type { Iエクスプローラー配線 } from './エクスプローラーパネル.ts'

describe('エクスプローラー配線ポートの結合テスト', () => {
    it('配線インターフェースを通じて大域世界・チャンク・マテリアル・使い方のオープン要求を受け取れること', () => {
        let 大域世界開いた = false
        let マテリアル開いた = false
        let 使い方開いた = false
        const 開いたチャンク座標: チャンク座標[] = []

        const 配線: Iエクスプローラー配線 = {
            on大域世界を開く: () => {
                大域世界開いた = true
            },
            onチャンクを開く: (座標) => {
                開いたチャンク座標.push(座標)
            },
            onマテリアルを開く: () => {
                マテリアル開いた = true
            },
            on使い方を開く: () => {
                使い方開いた = true
            },
        }

        配線.on大域世界を開く()
        配線.onチャンクを開く({ x: 1, z: 2 })
        配線.onマテリアルを開く()
        配線.on使い方を開く()

        assert.strictEqual(大域世界開いた, true)
        assert.strictEqual(開いたチャンク座標.length, 1)
        assert.strictEqual(開いたチャンク座標[0]?.x, 1)
        assert.strictEqual(開いたチャンク座標[0]?.z, 2)
        assert.strictEqual(マテリアル開いた, true)
        assert.strictEqual(使い方開いた, true)
    })
})
