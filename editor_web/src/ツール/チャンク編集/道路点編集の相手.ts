import type { Object3D } from 'three'
import type { 編集コマンド } from '../../生成/編集資源契約.ts'
import type { 道路点の在り処 } from './編集モデル/index.ts'

// 道路点のドラッグ移動とクリック編集が話しかける相手の、最小の口をまとめた宣言。
// チャンク編集と大域編集は状態クラスもビュー部品も別の型だが手順は同じであるため、
// 満たすべき口だけをここで決めて、1組のハンドラを両方のツールから使う。

// 触る三次元ビューの部分。マーカーは当たり判定、地形は移動先の取得、帯は挿入位置の取得に使う。
// 求めるのは当たりを道路の位置へ読み替える口だけであり、描画の中身までは求めない。

// 当たりの主とは、レイキャストの当たりが指す先として同じものかを見分けられる相手のことである。
// SengenThreeの三次元部品はこの形を満たす。ここで形だけを求めるのは、道路の当たりの読み替えが
// 描画の中身に依らないことを型で示すためである。
export interface 当たりの主 {
    readonly 実体: Object3D
}

export type 道路点マーカーの当たり = 当たりの主 & {
    当たった道路点の在り処を求める(交差した物体: Object3D): 道路点の在り処 | null
}

export type 路面の当たり = 当たりの主 & {
    当たった路面の道路添字を求める(交差した物体: Object3D): number | null
}

// 当たりの記録とは、レイキャストが返す当たり1件のうち、道路の編集が読む部分だけを表したもののことである。
// SengenThreeの部品交差情報はこの形を満たす。
export interface 当たりの記録 {
    readonly 部品: 当たりの主
    readonly 交差点: { readonly x: number; readonly y: number; readonly z: number }
    readonly 原初交差情報: { readonly object: Object3D }
}

export interface 道路点編集対象ビュー {
    readonly 道路点マーカー: 道路点マーカーの当たり
    readonly 道路帯: 路面の当たり
    readonly 地形: 当たりの主
}

// ドラッグとクリックの間に書き換えるUI状態。両ツールの編集状態クラスが満たす。
// アクティブな道路の添字は、道を描き足す先であり、パネルの幅などの設定が効く相手でもある。
// nullは「次に地形をクリックしたら新しい道を1本始める」ことを表す。
export interface 道路点の選択状態 {
    アクティブな道路の添字: number | null
    選択中の道路点: 道路点の在り処 | null
    つかんでいる道路点: 道路点の在り処 | null
}

export interface 道路点編集の操作先 {
    コマンドを実行する(コマンド: 編集コマンド): void
}

export interface 道路点編集の同期先 {
    道路を同期する(): void
    UIを同期する(): void
}
