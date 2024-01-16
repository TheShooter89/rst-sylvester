-- Create users table.
create table if not exists users
(
    id integer primary key not null,
    username text not null unique,
    password text not null
);

-- Insert "ferris" user.
insert into users (id, username, password)
values (1, 'ferris', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');

-- Insert "ferris" user.
insert into users (id, username, password)
values (2, 'tng_padova', '$argon2id$v=19$m=19456,t=2,p=1$WdDEcymi15daz05bl7NoEw$egEHohVmklmk1ExVTuL+ocMpoXvpafvIG4oq6l4KjoU');
