insert into maker(name)
select ?1
where not exists(select * from maker where name = ?1);

insert into director(name)
select ?1
where not exists(select * from director where name = ?1);

insert into smov(name, path, len, created, modified, extension, format, makers_id,
                 series_id, directors_id)
select ?1,
       ?2,
       ?3,
       ?4,
       ?5,
       ?6,
       ?7,
       ?8,
       ?9,
       ?10
where not exists(select * from smov where format = ?11);

