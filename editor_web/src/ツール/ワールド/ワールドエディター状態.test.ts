import { describe, it } from "node:test"
import assert from "node:assert/strict"
import { ワールドエディター状態 } from "./ワールドエディター状態.ts"
import type { 差し戻し断片 } from "./操作コマンド/index.ts"

describe("ワールドエディター状態の履歴スタック上限", () => {
    it("履歴スタック上限(50件)を超えて積むと最古の断片が破棄されること", () => {
        const 状態 = new ワールドエディター状態()
        const 上限 = ワールドエディター状態.履歴スタック上限

        for (let i = 0; i < 上限 + 10; i++) {
            const ダミー断片: 差し戻し断片 = {
                種類: "造成筆致差し戻し",
                値: {
                    チャンク座標: { x: 0, z: 0 },
                    旧高さデータ: new Float32Array([i]),
                },
            }
            状態.取り消し断片を積む(ダミー断片)
        }

        assert.strictEqual(状態.取り消し履歴スタック.length, 上限, "スタック長は上限を超えないべき")

        const 最古断片 = 状態.取り消し履歴スタック[0]
        assert.ok(最古断片 !== undefined)
        if (最古断片.種類 === "造成筆致差し戻し") {
            assert.strictEqual(最古断片.値.旧高さデータ[0], 10, "最古の10件は破棄されi=10が先頭にあるべき")
        }

        const 最新断片 = 状態.取り消し断片を取り出す()
        assert.ok(最新断片 !== undefined)
        if (最新断片.種類 === "造成筆致差し戻し") {
            assert.strictEqual(最新断片.値.旧高さデータ[0], 上限 + 9, "最新の断片が取り出せるべき")
        }
    })
})
