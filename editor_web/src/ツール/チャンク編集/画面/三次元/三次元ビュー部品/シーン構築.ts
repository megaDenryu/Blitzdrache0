import { 場面を作る, 透視カメラを作る, 環境光を作る, 平行光源を作る } from 'SengenThree'
import type { ワールド編集状態 } from '../../../編集モデル/index.ts'
import { 地形メッシュ部品 } from '../地形/地形メッシュ部品.ts'
import { ブラシリング部品 } from '../ブラシ/ブラシリング部品.ts'
import { 道路帯メッシュ部品 } from '../道路/道路帯メッシュ部品.ts'
import { 道路ノードメッシュ部品 } from '../道路/道路ノードメッシュ部品.ts'
import { 建物メッシュ部品 } from '../建物/建物メッシュ部品.ts'
import { 散布個体群部品 } from '../散布/散布個体群部品.ts'
import { 軌道カメラ制御器 } from '../カメラ/軌道カメラ制御器.ts'

export interface チャンクシーン部品束 {
    readonly 場面: ReturnType<typeof 場面を作る>
    readonly カメラ: ReturnType<typeof 透視カメラを作る>
    readonly カメラ制御: 軌道カメラ制御器
    readonly 地形: 地形メッシュ部品
    readonly ブラシリング: ブラシリング部品
    readonly 道路帯: 道路帯メッシュ部品
    readonly 道路ノード: 道路ノードメッシュ部品
    readonly 建物: 建物メッシュ部品
    readonly 散布: 散布個体群部品
}

// チャンク編集ビューの三次元シーングラフを構築する。呼び出し元(三次元ビュー部品)は
// キャンバス・描画ループ・破棄の管理だけを担い、シーンの内訳の組み立てはここへ閉じる。
export function チャンクシーンを構築する(編集状態: ワールド編集状態, 初期背景色: string | number): チャンクシーン部品束 {
    const チャンク = 編集状態.チャンク一覧マップ.values().next().value
    if (!チャンク) throw new Error('初期チャンクが登録されていません')

    const 地形 = new 地形メッシュ部品(チャンク.高さ場, チャンク.地表材質)
    const ブラシリング = new ブラシリング部品()
    const 道路帯 = new 道路帯メッシュ部品()
    const 道路ノード = new 道路ノードメッシュ部品()
    const 建物 = new 建物メッシュ部品()
    const 散布 = new 散布個体群部品()

    const カメラ = 透視カメラを作る({ 画角: 50, アスペクト比: 16 / 9, 奥クリップ距離: 2000 })
    const カメラ制御 = new 軌道カメラ制御器(カメラ)

    const 場面 = 場面を作る()
        .背景色を設定する(初期背景色)
        .childs([
            カメラ,
            環境光を作る({ 色: 0xe2e8f0, 強さ: 0.6 }),
            平行光源を作る({ 色: 0xffedd5, 強さ: 1.2 }).位置を設定する(120, 200, 80),
            地形,
            ブラシリング,
            道路帯,
            道路ノード,
            建物,
            散布,
        ])

    return { 場面, カメラ, カメラ制御, 地形, ブラシリング, 道路帯, 道路ノード, 建物, 散布 }
}
