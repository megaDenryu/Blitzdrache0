// サーバーとの通信結果を表す判別共用体。
// 資源が存在しない状態（200 null または 204 No Content）を「無し」として型安全に区別する。
// 参照: `crates/editor_server/src/routes/`

export interface サーバーエラー {
    readonly 種別: string
    readonly 説明: string
}

export type 読込結果<T> =
    | { readonly 種別: '成功'; readonly 値: T }
    | { readonly 種別: '無し' }
    | { readonly 種別: '失敗'; readonly エラー: サーバーエラー }

export type 保存結果 =
    | { readonly 種別: '成功' }
    | { readonly 種別: '失敗'; readonly エラー: サーバーエラー }

export function 読込成功<T>(値: T): 読込結果<T> {
    return { 種別: '成功', 値 }
}

export function 読込無し<T>(): 読込結果<T> {
    return { 種別: '無し' }
}

export function 読込失敗<T>(エラー: サーバーエラー): 読込結果<T> {
    return { 種別: '失敗', エラー }
}

export function 保存成功(): 保存結果 {
    return { 種別: '成功' }
}

export function 保存失敗(エラー: サーバーエラー): 保存結果 {
    return { 種別: '失敗', エラー }
}
