```sh
nix-shell -p rustup openssl jq nodejs sqlite
```

[archive](https://xkcd.com/archive/)
```js
let urls = [];
document.querySelectorAll('#middleContainer a').forEach(a => {
    urls.push({
		num: a.href.match(/\d+/)[0],
		url: `${a.href}info.0.json`
    });
});
console.debug(JSON.stringify(urls));
```
[comics.json](comics.json)

```sh
jq . comics.json > comics-pretty.json
```
[comics-pretty.json](comics-pretty.json)

[download.js](download.js)
```sh
node download.js
```

```sh
sqlite3 comics.db
sqlite> .read schema.sql
sqlite> .read insert.sql
```