// 実サーバー接続のテストが共有するfetchモックの差し込み。
// globalThis.fetchを差し替え、最後に送られた要求URL・要求オプションを取得できるようにし、
// 返す応答はテストごとに差し替えられる。復元まで含めてこの型が受け持つ。
export interface モックFetch制御 {
    最後の要求URLを得る(): string
    最後の要求オプションを得る(): RequestInit | undefined
    レスポンスを差し替える(応答: Response): void
    元のFetchへ戻す(): void
}

export function モックFetchを差し込む(): モックFetch制御 {
    const 元のFetch = globalThis.fetch
    let 最後の要求URL = ''
    let 最後の要求オプション: RequestInit | undefined
    let 現在の応答: Response = new Response('null', { status: 200 })

    globalThis.fetch = async (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
        最後の要求URL = String(input)
        最後の要求オプション = init
        return 現在の応答
    }

    return {
        最後の要求URLを得る: () => 最後の要求URL,
        最後の要求オプションを得る: () => 最後の要求オプション,
        レスポンスを差し替える: (応答) => {
            現在の応答 = 応答
        },
        元のFetchへ戻す: () => {
            globalThis.fetch = 元のFetch
        },
    }
}
