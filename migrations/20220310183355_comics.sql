CREATE TABLE comics (
    number      INTEGER PRIMARY KEY NOT NULL,
    image       TEXT                NOT NULL,
    publication TEXT                NOT NULL,
    title       TEXT                NOT NULL,
    title_safe  TEXT                NOT NULL,
    alternate   TEXT                NOT NULL,
    link        TEXT                NOT NULL,
    transcript  TEXT                NOT NULL,
    news        TEXT                NOT NULL
) STRICT;
