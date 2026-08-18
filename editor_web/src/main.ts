import { 外殻レイアウト } from 'VscodeShellLayout'

// コンポジションルート。実装の注入をここ1箇所へ集める。
// #app へのマウントだけは、SengenUI/VscodeShellLayoutの外側（アプリ起動点）として
// DOM APIを直接使う（この構成における唯一の例外）。
// この段(段1)ではツール登録簿・場所の断片ルーターをまだ持たないため、
// 外殻レイアウトを最小構成でそのまま表示する。ツールの配線は段4で足す。
function 起動する(): void {
    const アプリ要素 = document.getElementById('app')
    if (アプリ要素 === null) {
        throw new Error('#app が見つからない')
    }

    const シェル = new 外殻レイアウト({
        タイトル: 'Blitzdrache0 エディター',
        アクティビティ項目一覧: [],
    })
    アプリ要素.appendChild(シェル.dom.element)
}

起動する()
