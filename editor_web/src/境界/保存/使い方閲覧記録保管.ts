// ブラウザのlocalStorageを使い方ガイドの閲覧記録の永続化先として使う境界。
// 初回起動時だけ使い方タブを自動で開く(Welcomeページ方式)ために、閲覧済みかどうかを記録する。

const 保管キー = 'BLITZDRACHE_EDITOR_GUIDE_VIEWED'

export function 使い方の閲覧記録を書く(): void {
    try {
        localStorage.setItem(保管キー, '1')
    } catch {
        // localStorageが使えない環境では控えを持たない
    }
}

export function 使い方を閲覧済みか(): boolean {
    try {
        return localStorage.getItem(保管キー) === '1'
    } catch {
        return false
    }
}
