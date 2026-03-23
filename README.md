# pylzx

Rust製のLZXD (LZX) デコーダをPythonから使えるモジュールです。

## インストール

### 開発インストール

```bash
pip install maturin
maturin develop
```

### wheelビルド

```bash
maturin build --release
```

## Python API (LZXデコード専用)

### 1. 状態保持デコーダ (`LzxdDecoder`)

```python
from pylzx import LzxdDecoder

decoder = LzxdDecoder(window_size=65536)
decoded = decoder.decompress_next(chunk_bytes, output_len)
```

- `window_size`: 32768, 65536, 131072, ... 33554432
- `decompress_next(chunk, output_len)`: 1チャンク分をデコード
- `reset()`: デコーダ状態を初期化

### 2. 連続チャンクデコード (`decompress_lzxd_chunks`)

```python
from pylzx import decompress_lzxd_chunks

raw = decompress_lzxd_chunks(
    chunks=compressed_chunks,
    output_lengths=chunk_output_sizes,
    window_size=65536,
)
```

- `chunks`: 圧縮チャンクの配列 (`list[bytes]`)
- `output_lengths`: 各チャンクの復号後サイズ (`list[int]`)
- `chunks` と `output_lengths` は同じ長さである必要があります
