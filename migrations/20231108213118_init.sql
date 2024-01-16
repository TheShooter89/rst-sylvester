-- Create users table.
create table if not exists users
(
    id integer primary key not null,
    username text not null unique,
    password text not null
);

-- Insert "ranghi" user, pwd "latini".
insert into users (id, username, password)
values (1, 'ranghi', '$argon2id$v=19$m=19456,t=2,p=1$gHZ82/l2eha46TH9XWwUdQ$TgvNvy2xnwfKKfN69Kch7/RtJu2qx0hIH40L4vKHN7s');

-- Insert "tanque" user, pwd "slavaukraini".
insert into users (id, username, password)
values (2, 'tanque', '$argon2id$v=19$m=19456,t=2,p=1$g82u5m59TiDxumUj9lB1pg$IlZ2OAFArNdKP0DZEw25o0HRLJp1f6tmF6U7v3jWPmc');
