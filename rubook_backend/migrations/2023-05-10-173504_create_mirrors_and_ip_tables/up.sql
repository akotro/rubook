-- Your SQL goes here
CREATE TABLE ip_blacklist (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    ip_address VARCHAR(15) NOT NULL,
    UNIQUE (ip_address)
);

INSERT INTO ip_blacklist (ip_address)
VALUES
    ('103.178.228.27'),
    ('111.173.118.83'),
    ('111.85.200.64'),
    ('114.100.177.182'),
    ('119.60.105.179'),
    ('124.90.215.107'),
    ('139.170.202.166'),
    ('144.255.16.185'),
    ('152.89.196.144'),
    ('159.203.192.14'),
    ('159.203.208.12'),
    ('162.142.125.215'),
    ('162.243.151.4'),
    ('167.94.146.58'),
    ('167.99.141.170'),
    ('171.118.64.11'),
    ('182.54.4.215'),
    ('183.136.225.9'),
    ('184.105.139.124'),
    ('184.105.139.72'),
    ('184.105.139.76'),
    ('185.180.143.188'),
    ('185.180.143.50'),
    ('188.26.198.163'),
    ('192.155.90.118'),
    ('198.235.24.205'),
    ('198.235.24.215'),
    ('20.187.65.106'),
    ('216.146.25.63'),
    ('219.143.174.49'),
    ('3.239.85.113'),
    ('74.234.160.134'),
    ('78.108.177.51'),
    ('80.71.157.4'),
    ('87.236.176.161'),
    ('94.183.49.9'),
    ('94.236.135.131'),
    ('95.214.53.99');

 CREATE TABLE mirrors (
    id                   INTEGER PRIMARY KEY auto_increment,
    host_url             TEXT NOT NULL,
    search_url           TEXT,
    search_url_fiction   TEXT,
    download_url         TEXT,
    download_url_fiction TEXT,
    download_pattern     TEXT,
    sync_url             TEXT,
    cover_pattern        TEXT
);

INSERT INTO mirrors (
    host_url,
    search_url,
    search_url_fiction,
    sync_url,
    cover_pattern
)
VALUES (
    'http://libgen.is/',
    'https://libgen.is/search.php',
    'https://libgen.is/fiction',
    'http://libgen.is/json.php',
    'http://libgen.is/covers/{cover-url}'
);

INSERT INTO mirrors (
    host_url,
    search_url,
    search_url_fiction,
    sync_url,
    cover_pattern
)
VALUES (
    'http://libgen.rs/',
    'https://libgen.rs/search.php',
    'https://libgen.rs/fiction',
    'http://libgen.rs/json.php',
    'http://libgen.rs/covers/{cover-url}'
);

INSERT INTO mirrors (
    host_url,
    search_url,
    search_url_fiction,
    sync_url,
    cover_pattern
)
VALUES (
    'http://libgen.st/',
    'https://libgen.st/search.php',
    'https://libgen.st/fiction',
    'http://libgen.st/json.php',
    'http://libgen.st/covers/{cover-url}'
);

INSERT INTO mirrors (
    host_url,
    download_url,
    download_pattern,
    download_url_fiction,
    sync_url,
    cover_pattern
)
VALUES (
    'http://library.lol/',
    'http://library.lol/main/{md5}',
    'http://library.lol/main/{md5}',
    'http://library.lol/fiction/{md5}',
    'http://libgen.rs/json.php',
    'http://libgen.rs/covers/{cover-url}'
);

INSERT INTO mirrors (
    host_url,
    download_url,
    download_pattern,
    download_url_fiction,
    sync_url,
    cover_pattern
)
VALUES (
    'http://libgen.lc/',
    'http://libgen.lc/get.php?md5={md5}',
    'http://libgen.lc/get.php?md5={md5}',
    'http://libgen.lc/get.php?md5={md5}',
    'http://libgen.ls/json.php',
    'http://libgen.lc/covers/{cover-url}'
);

INSERT INTO mirrors (
    host_url,
    download_url,
    download_pattern,
    download_url_fiction
)
VALUES (
    'https://libgen.rocks/',
    'https://libgen.rocks/ads.php?md5={md5}',
    'https://libgen.rocks/ads.php?md5={md5}',
    'https://libgen.rocks/ads.php?md5={md5}'
);

INSERT INTO mirrors (
    host_url,
    download_url,
    download_pattern,
    download_url_fiction
)
VALUES (
    'https://libgen.me/',
    'https://libgen.me/book/{md5}',
    'https://libgen.me/book/{md5}',
    'https://libgen.me/book/{md5}'
);
