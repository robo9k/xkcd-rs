INSERT INTO comics SELECT
    json_extract(value, '$.alt'),
    json_extract(value, '$.day'),
    json_extract(value, '$.img'),
    json_extract(value, '$.link'),
    json_extract(value, '$.month'),
    json_extract(value, '$.news'),
    json_extract(value, '$.num'),
    json_extract(value, '$.safe_title'),
    json_extract(value, '$.title'),
    json_extract(value, '$.transcript'),
    json_extract(value, '$.year')
FROM json_each(readfile('db/xkcd-1.json'));
