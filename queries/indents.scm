; taken from https://github.com/zed-industries/zed/blob/6657e301cd0ee9e7b7b5352957ef30728ae2a874/crates/languages/src/python/indents.scm
(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
