// 楽曲1件の定義を指す安定識別子。数学DDDのbranded type(型レベルのみ、ランタイムコストゼロ)で
// stringと区別し、他の裸のstring(曲の表示名やパターンの名乗り等、意味の違う識別子)との取り違えを防ぐ。
// branded型はstringの部分型であるため、生のstringが要る場所(URL組み立て・JSON直列化)へは
// 変換なしでそのまま渡せる。逆方向(検証されていないstring→楽曲ID)だけがこの生成関数を通る。
// 通信境界(fetch/JSON)の内側でJSON.parseした直後の生のstringから型を得るのも、この生成関数である。
export type 楽曲ID = string & { readonly __brand: '楽曲ID' }

// 空文字はどの楽曲定義も指さない無効値であり、生成時に拒む(無言のデフォルト適用を避ける)。
export function 楽曲IDを生成する(綴り: string): 楽曲ID {
    if (綴り === '') throw new Error('楽曲IDは空文字を受け付けない')
    return 綴り as 楽曲ID
}
