import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { チャンク座標 } from '../../境界/通信/index.ts'
import type { Iエクスプローラー配線 } from './エクスプローラーパネル.ts'

describe('エクスプローラー配線ポートの結合テスト', () => {
    it('配線インターフェースを通じて大域世界とチャンクのオープン要求を受け取れること', () => {
        let 大域世界開いた = false
        const 開いたチャンク座標: チャンク座標[] = []

        const 配線: Iエクスプローラー配線 = {
            on大域世界を開く: () => {
                大域世界開いた = true
            },
            onチャンクを開く: (座標) => {
                開いたチャンク座標.push(座標)
            },
        }

        配線.on大域世界を開く()
        配線.onチャンクを開く({ x: 1, z: 2 })

        assert.strictEqual(大域世界開いた, true)
        assert.strictEqual(開いたチャンク座標.length, 1)
        assert.strictEqual(開いたチャンク座標[0]?.x, 1)
        assert.strictEqual(開いたチャンク座標[0]?.z, 2)
    })
})
