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

[schema.sql](schema.sql)
[insert.sql](insert.sql)
```sh
sqlite3 comics.db
sqlite> .read schema.sql
sqlite> .read insert.sql
```

---

```sh
jq -r 'keys | @csv' db/xkcd-1.json && jq -r '[.alt,.day,.img,.link,.month,.news,.num,.safe_title,.title,.transcript,.year] | @csv' db/xkcd-*.json > comics.csv
sqlite3 comics.db
sqlite> .mode csv
sqlite> .import comics.csv comics
```
[comics.csv](comics.csv)

* https://xkcd.com/1005/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/1037/info.0.json object ({"pre":"<no...) is not valid in a csv row
* https://xkcd.com/1110/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/1190/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/1193/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/1331/info.0.json object ({"headerext...) is not valid in a csv row
* https://xkcd.com/1335/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/1350/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/1416/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/1446/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/1506/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/1525/info.0.json object ({"headerext...) is not valid in a csv row
* https://xkcd.com/1608/info.0.json object ({"headerext...) is not valid in a csv row
* https://xkcd.com/1663/info.0.json object ({"headerext...) is not valid in a csv row
* https://xkcd.com/1975/info.0.json object ({"pre":"","...) is not valid in a csv row
* https://xkcd.com/2067/info.0.json object ({"headerext...) is not valid in a csv row
* https://xkcd.com/2198/info.0.json object ({"headerext...) is not valid in a csv row
* https://xkcd.com/2288/info.0.json object ({"headerext...) is not valid in a csv row
* https://xkcd.com/2445/info.0.json object ({"headerext...) is not valid in a csv row
* https://xkcd.com/826/info.0.json object ({"pre":"\n<...) is not valid in a csv row

```rust
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = Url::parse("https://xkcd.com")?;
    let url = base.join("//xkcd.com/1506/")?;
    println!("{}", String::from(url));

    Ok(())
}
```

```rust
use http::Uri;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri: Uri = "//xkcd.com/1506/".parse()?;
    println!("{}", uri);

    Ok(())
}
```
