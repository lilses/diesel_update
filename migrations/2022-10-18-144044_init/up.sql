-- Your SQL goes here

create table table_1
(
    id             serial
            primary key,
    confirm1       boolean   default false             not null,
    confirm2       boolean   default false             not null
);

create table table_2
(
    id             serial
            primary key,
    confirm1       boolean   default false             not null,
    confirm2       boolean   default false             not null
);

