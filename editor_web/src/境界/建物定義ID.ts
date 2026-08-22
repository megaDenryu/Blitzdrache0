// 建物1件の定義を指す安定識別子。数学DDDのbranded type(型レベルのみ、ランタイムコストゼロ)で
// stringと区別し、他の裸のstring(建物の配置の識別子等、意味の違う識別子)との取り違えを防ぐ。
// branded型はstringの部分型であるため、生のstringが要る場所(URL組み立て・JSON直列化)へは
// 変換なしでそのまま渡せる。逆方向(検証されていないstring→建物定義ID)だけがこの生成関数を通る。
// 通信境界(fetch/JSON)の内側でJSON.parseした直後の生のstringから型を得るのも、この生成関数である。
// 参照: `_doc/計画/ユビキタス言語.md`「ゲーム開発用エディターの語彙」
export type 建物定義ID = string & { readonly __brand: '建物定義ID' }

// 空文字はどの建物定義も指さない無効値であり、生成時に拒む(無言のデフォルト適用を避ける)。
export function 建物定義IDを生成する(綴り: string): 建物定義ID {
    if (綴り === '') throw new Error('建物定義IDは空文字を受け付けない')
    return 綴り as 建物定義ID
}
