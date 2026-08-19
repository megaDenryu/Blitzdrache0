// ブラウザのlocalStorageをエディターテーマ選択の永続化先として使う境界。
// 直前に選択されたカラーテーマを次回起動時に復元するために使用する。

import type { テーマ名 } from '../テーマ/テーマ型.ts'

const 保管キー = 'BLITZDRACHE_EDITOR_THEME'

export type テーマ保管の読み出し結果 =
    | { readonly 種別: '控えあり'; readonly テーマ名: テーマ名 }
    | { readonly 種別: '控えなし' }
    | { readonly 種別: '形式不正'; readonly 説明: string }

export function テーマの控えを書く(名前: テーマ名): void {
    try {
        localStorage.setItem(保管キー, 名前)
    } catch {
        // localStorageが使えない環境では控えを持たない
    }
}

export function テーマの控えを読む(): テーマ保管の読み出し結果 {
    let 生文字列: string | null = null
    try {
        生文字列 = localStorage.getItem(保管キー)
    } catch {
        return { 種別: '控えなし' }
    }
    if (!生文字列) return { 種別: '控えなし' }

    if (生文字列 === '工房' || 生文字列 === '夜間') {
        return { 種別: '控えあり', テーマ名: 生文字列 }
    }

    try {
        const 解析 = JSON.parse(生文字列)
        if (解析 === '工房' || 解析 === '夜間') {
            return { 種別: '控えあり', テーマ名: 解析 }
        }
        if (typeof 解析 === 'object' && 解析 !== null && (解析.テーマ名 === '工房' || 解析.テーマ名 === '夜間')) {
            return { 種別: '控えあり', テーマ名: 解析.テーマ名 }
        }
    } catch {
        // JSON parse失敗
    }

    return { 種別: '形式不正', 説明: `未知のテーマ名: ${生文字列}` }
}

export function テーマの控えを消す(): void {
    try {
        localStorage.removeItem(保管キー)
    } catch {
        // 何もしない
    }
}
