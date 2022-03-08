create table if not exists smov
(
    id           integer primary key autoincrement,
    name         TEXT,
    filename     TEXT,
    realname     TEXT,
    seekname     TEXT,
    path         TEXT,
    len          integer,
    created      integer,
    modified     integer,
    extension    TEXT,
    format       TEXT,
    release_time integer,
    duration     integer,
    publisher_id integer,
    makers_id    integer              Null,
    series_id    integer              Null,
    directors_id integer              Null,
    is_retrieve  TINYINT(1) Default 0 Null,
    is_active    TINYINT(1) Default 0 Null,
    isch         TINYINT(1) Default 0 Null,   --什么意思？？？ 是否中文。。。
);



Create Table if not exists actor
(
    id   integer primary key autoincrement,
    name TEXT
);

Create Table if not exists director
(
    id   integer primary key autoincrement,
    name TEXT
);

Create Table if not exists maker
(
    id   integer primary key autoincrement,
    name TEXT
);

Create Table if not exists serie
(
    id   integer primary key autoincrement,
    name TEXT
);

Create Table if not exists tag
(
    id   integer primary key autoincrement,
    name TEXT
);

Create Table if not exists publisher
(
    id   integer primary key autoincrement,
    name TEXT
);

Create Table if not exists smov_actor
(
    id       integer primary key autoincrement,
    smov_id  integer,
    actor_id integer
);

Create Table if not exists smov_tag
(
    id      integer primary key autoincrement,
    smov_id integer,
    tag_id  integer
);

Create Table if not exists folder
(
    id   integer primary key autoincrement,
    path TEXT
);

Create Table if not exists sys
(
    id      integer primary key autoincrement,
    version TEXT,
    journal TEXT,
    updated integer
);

Create Table if not exists sys_folder
(
    id      integer primary key autoincrement,
    path    TEXT
);