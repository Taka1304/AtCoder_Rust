#!/bin/bash

# 引数のチェック
if [ "$#" -ne 1 ]; then
  echo "使用法: $0 数字"
  exit 1
fi

number=$1

# ディレクトリを作成
dir="src/abc$number"
mkdir -p $dir

# ファイルを作成
touch "$dir/a.rs" "$dir/b.rs" "$dir/c.rs" "$dir/d.rs" "$dir/e.rs"

echo "Done: $dir"
