import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 編集コマンドを適用する } from './index.ts'
import {
    見下ろし図の試験のチャンク座標 as チャンク座標,
    起伏のあるチャンクの編集状態を作る,
    見下ろし図の8枝を通るコマンド列,
} from './見下ろし図の試験の下ごしらえ.ts'

describe('見下ろし図コマンドの決定性', () => {
    it('同じコマンド列を同じ初期状態へ2回適用した結果の高さ場・材質・下書きが一致すること', () => {
        const 状態1 = 起伏のあるチャンクの編集状態を作る()
        const 状態2 = 起伏のあるチャンクの編集状態を作る()
        for (const コマンド of 見下ろし図の8枝を通るコマンド列()) {
            編集コマンドを適用する(状態1, コマンド)
            編集コマンドを適用する(状態2, コマンド)
        }
        const チャンク1 = 状態1.チャンクを取得する(チャンク座標)
        const チャンク2 = 状態2.チャンクを取得する(チャンク座標)
        assert.deepStrictEqual(new Uint8Array(チャンク1.高さ場.格子データ.buffer), new Uint8Array(チャンク2.高さ場.格子データ.buffer), '高さ場がバイト一致するべき')
        assert.deepStrictEqual(new Uint8Array(チャンク1.地表材質.材質データ.buffer), new Uint8Array(チャンク2.地表材質.材質データ.buffer), '地表材質がバイト一致するべき')
        assert.deepStrictEqual(チャンク1.下書き.契約の形で取り出す(), チャンク2.下書き.契約の形で取り出す(), '下書きが一致するべき')
        assert.strictEqual(チャンク1.構造を取得する().見下ろし図の下書き.粗マスの塗り一覧.length, 64, '構造の取得に下書きが乗るべき')
    })
})
