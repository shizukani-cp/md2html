# md2html
## 仕様
カレントディレクトリにtemplate.htmlがあれば、そこにあるプレースホルダー({})にMarkdownをHTMLにしたものが入ります。
template.htmlがなければ、テンプレートには<html>\n<head></head>\n<body>{}</body>\n</html>が使われます。
## 使い方
```sh
md2html your_markdown_file.md
```