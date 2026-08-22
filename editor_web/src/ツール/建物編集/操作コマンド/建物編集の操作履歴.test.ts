import { strictEqual } from 'node:assert/strict'
import { describe, it } from 'node:test'
import { 初期の建物の格子を作る } from '../編集モデル/index.ts'
import { 建物編集の操作履歴 } from './建物編集の操作履歴.ts'

// 1回の触りが1つのコマンドとして積まれ、取り消しとやり直しで元の格子へ戻ることを確かめる。
describe('建物編集の操作履歴', () => {
    const 初期の格子 = (): ReturnType<typeof 初期の建物の格子を作る> => 初期の建物の格子を作る('grid_building_1', '新しい建物')

    it('何も積んでいなければ取り消しもやり直しもできない', () => {
        const 履歴 = new 建物編集の操作履歴(初期の格子())
        strictEqual(履歴.取り消せるか, false)
        strictEqual(履歴.やり直せるか, false)
    })

    it('升目を置いてから取り消すと升目の数が戻る', () => {
        const 履歴 = new 建物編集の操作履歴(初期の格子())
        strictEqual(履歴.操作を1つ積む('升目を置く', (モデル) => モデル.升目を置く({ 横: 1, 奥: 0, 階: 0 })), true)
        strictEqual(履歴.モデル.升目を昇順に並べる().length, 2)
        strictEqual(履歴.取り消す(), true)
        strictEqual(履歴.モデル.升目を昇順に並べる().length, 1)
        strictEqual(履歴.やり直す(), true)
        strictEqual(履歴.モデル.升目を昇順に並べる().length, 2)
    })

    it('何も変えない操作は履歴へ積まない', () => {
        const 履歴 = new 建物編集の操作履歴(初期の格子())
        strictEqual(履歴.操作を1つ積む('升目を置く', (モデル) => モデル.升目を置く({ 横: 0, 奥: 0, 階: 0 })), false)
        strictEqual(履歴.取り消せるか, false)
    })

    it('取り消したあとに新しい操作を積むとやり直しの先が捨てられる', () => {
        const 履歴 = new 建物編集の操作履歴(初期の格子())
        履歴.操作を1つ積む('升目を置く', (モデル) => モデル.升目を置く({ 横: 1, 奥: 0, 階: 0 }))
        履歴.取り消す()
        履歴.操作を1つ積む('升目を置く', (モデル) => モデル.升目を置く({ 横: 0, 奥: 1, 階: 0 }))
        strictEqual(履歴.やり直せるか, false)
        strictEqual(履歴.モデル.升目があるか({ 横: 1, 奥: 0, 階: 0 }), false)
        strictEqual(履歴.モデル.升目があるか({ 横: 0, 奥: 1, 階: 0 }), true)
    })
})
