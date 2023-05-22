# Mychat-GPT
![mychat-gpt Demo](https://github.com/fujinov/Mychat-GPT/assets/98008144/5c3073a3-ac14-4463-990c-766d81048d7b)

## 概要
コマンドラインで使用するChatGPT

## 特徴
- ストリーム表示と一括表示の切り替え
- チャットログの保存（マークダウン形式。見出しをつけた簡単なもの）

## 必要なもの
- Rust1.64.0+
- OpenAIのAPIキー

## 導入
1. ```git clone https://github.com/fujinov/Mychat-GPT.git```
2. APIキーの保存（どちらか）
    - 環境変数「**OPENAI_API_KEY**」
    - cargo runで実行するなら「src」と同じ階層にファイル「**\.apikey**」を作成してその中に保存。バイナリで実行なら、バイナリと同階層に上記ファイルを作成して保存。
3. ```cargo run```もしくは```cargo build```

## 使い方
```mychat-gpt[.exe] [OPTIONS] [ROLE]```

### 引数\[ROLE\]
**GPTの"role": "system"の設定**<br>
```mychat-gpt 英語で返答してください```

### オプション\[OPTIONS\]
**-m, --model \<MODEL\>**<br>
モデルの指定。省略した場合のデフォルトは「**gpt-3.5-turbo**」<br>
```mychat-gpt -m gpt-4```

**-l, --lines**<br>
入力モードの終了をエンドオブファイル（EOF）が来るまでに変更。EOFを挿入するにはWindowsは「**Ctrl+z**」、 Unix系は「**Ctrl+d**」

**-n, --nostream**<br>
ストリームをオフにして一括表示。一括表示のときには終了時に使用したトークンを表示。

**-h, --help**<br>
ヘルプの表示

### 入力時
チャットの終了
**:q　もしくは　空行を挿入**<br>

メッセージのリセット（"role": "system"はそのまま）
**:r**<br>

チャットを保存して終了
**:sq**<br>

チャットの保存とメッセージのリセット
**:sr**<br>

※チャットのログは「./chat/年月日」に。
