import { deepStrictEqual, ok, strictEqual } from 'node:assert/strict'
import { readFileSync, readdirSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, it } from 'node:test'
import { 建物の格子の編集モデル } from './建物の格子の編集モデル.ts'
import { 初期の建物の格子を作る } from './初期の建物の格子を作る.ts'
import { 立体の見取りを組み立てる, 見取りの外接箱を求める } from './建物の立体の見取り.ts'
import { ベイの奥行メートル, ベイの横幅メートル, 階の高さメートル } from './ベイの寸法.ts'

// 三次元の識別色表示が描く立体の並びが、升目の宣言のとおりに位置と役割を持つことを確かめる。
describe('建物の立体の見取り', () => {
    const 新しいモデル = (): 建物の格子の編集モデル =>
        建物の格子の編集モデル.契約の値から復元する(初期の建物の格子を作る('grid_building_1', '新しい建物'))

    it('升目1つが骨格と床と屋根2枚と壁4枚を出す', () => {
        const 見取り = 立体の見取りを組み立てる(新しいモデル().升目を昇順に並べる())
        const 役割ごとの数 = (役割: string): number => 見取り.filter((立体) => 立体.役割 === 役割).length
        strictEqual(役割ごとの数('骨格'), 1)
        strictEqual(役割ごとの数('床'), 1)
        strictEqual(役割ごとの数('屋根'), 2)
        strictEqual(役割ごとの数('壁'), 4)
        strictEqual(役割ごとの数('飾り'), 0)
    })

    it('隣へ升目を置くと骨格がベイの横幅だけ離れて並ぶ', () => {
        const モデル = 新しいモデル()
        モデル.升目を置く({ 横: 1, 奥: 0, 階: 0 })
        const 骨格一覧 = 立体の見取りを組み立てる(モデル.升目を昇順に並べる()).filter((立体) => 立体.役割 === '骨格')
        strictEqual(骨格一覧.length, 2)
        strictEqual(骨格一覧[1].中心メートル.x - 骨格一覧[0].中心メートル.x, ベイの横幅メートル)
    })

    it('階を積むと骨格が階の高さだけ上へ載る', () => {
        const モデル = 新しいモデル()
        モデル.升目を置く({ 横: 0, 奥: 0, 階: 1 })
        const 骨格一覧 = 立体の見取りを組み立てる(モデル.升目を昇順に並べる()).filter((立体) => 立体.役割 === '骨格')
        strictEqual(骨格一覧.length, 2)
        ok(Math.abs(骨格一覧[1].中心メートル.y - 骨格一覧[0].中心メートル.y - 階の高さメートル) < 1e-9)
    })

    it('壁へ飾りを付けると小箱が1つ増える', () => {
        const モデル = 新しいモデル()
        モデル.面の飾りを定める({ 横: 0, 奥: 0, 階: 0 }, '正面', { 種類: '出窓を差し込む' })
        const 見取り = 立体の見取りを組み立てる(モデル.升目を昇順に並べる())
        strictEqual(見取り.filter((立体) => 立体.役割 === '飾り').length, 1)
    })

    it('升目が1つも無い格子でも1ベイぶんの外接箱が決まる', () => {
        const 外接箱 = 見取りの外接箱を求める([])
        deepStrictEqual(外接箱.中心メートル, { x: 0, y: 階の高さメートル / 2, z: 0 })
        ok(外接箱.対角の長さメートル > ベイの奥行メートル)
    })
})

// 判断9のとおり、ブラウザは実部品のglbを配信せず、ベイの刻みの綴りを写しとして持つ。
// 写しが2つになると片方だけを直した食い違いが絵の中でしか見えないため、綴りが1ファイルに限ることを機械で守る。
describe('ベイの刻みの綴りの正本', () => {
    const 建物編集の根 = dirname(dirname(fileURLToPath(import.meta.url)))

    it('2.0と2.6の綴りはベイの寸法だけが持つ', () => {
        const 綴りを持つファイル = tsファイルを数え上げる(建物編集の根).filter((パス) => {
            const 本文 = readFileSync(パス, 'utf8')
            return /(^|[^.\d])2\.6(?![\d])/.test(本文) || /(^|[^.\d])2\.0(?![\d])/.test(本文)
        })
        deepStrictEqual(
            綴りを持つファイル.map((パス) => パス.split(/[\\/]/).pop()),
            ['ベイの寸法.ts'],
        )
    })

    function tsファイルを数え上げる(ディレクトリ: string): string[] {
        return readdirSync(ディレクトリ, { withFileTypes: true }).flatMap((項目) => {
            const パス = join(ディレクトリ, 項目.name)
            if (項目.isDirectory()) return tsファイルを数え上げる(パス)
            return 項目.name.endsWith('.ts') && !項目.name.endsWith('.test.ts') ? [パス] : []
        })
    }
})
