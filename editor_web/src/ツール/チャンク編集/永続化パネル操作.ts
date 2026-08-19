// 保存・読込・書き出しパネルが共有する「ボタン無効化→実行中文言→I/O→結果文言→再有効化」の
// 定型手順。チャンク編集・大域編集の各永続化配線・書き出し配線が使う。
export interface 進行状況表示パネル {
    ボタン活性状態を更新する(活性: boolean): void
    状態文言を更新する(文言: string, エラーか: boolean): void
}

export interface パネル操作結果 {
    readonly 文言: string
    readonly エラーか: boolean
}

export async function パネル操作を実行する(
    パネル: 進行状況表示パネル,
    実行中文言: string,
    処理: () => Promise<パネル操作結果>,
    例外接頭辞: string,
): Promise<void> {
    パネル.ボタン活性状態を更新する(false)
    パネル.状態文言を更新する(実行中文言, false)
    try {
        const { 文言, エラーか } = await 処理()
        パネル.状態文言を更新する(文言, エラーか)
    } catch (原因: unknown) {
        const メッセージ = 原因 instanceof Error ? 原因.message : String(原因)
        パネル.状態文言を更新する(`${例外接頭辞}: ${メッセージ}`, true)
    } finally {
        パネル.ボタン活性状態を更新する(true)
    }
}
