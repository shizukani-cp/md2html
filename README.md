# md2html
## 仕様
カレントディレクトリにtemplate.htmlがあれば、そこにあるプレースホルダー({})にMarkdownをHTMLにしたものが入ります。
template.htmlがなければ、テンプレートには
```html
<html>
<head></head>
<body>{}</body>
</html>
```
が使われます。
結果のファイルは、整形していないので、汚くなっていることがあります。
## 使い方
```sh
md2html your_markdown_file.md
```