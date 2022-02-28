const fs = require('fs');
const https = require('https');

let comics = JSON.parse(fs.readFileSync('comics-pretty.json'));
comics.forEach(comic => {
    https.get(comic.url, (res) => {
        const path = `xkcd-${comic.num}.json`;
        const writeStream = fs.createWriteStream(path);
        res.pipe(writeStream);
        writeStream.on('finish', () => writeStream.close());
    });
});
