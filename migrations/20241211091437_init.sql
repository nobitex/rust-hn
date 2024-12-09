-- Add migration script here

create table if not exists users (
    id serial primary key,
    username varchar(255) unique not null,
    password_salt varchar(255) not null,
    password_hash varchar(255) not null,
    is_verified boolean default false not null,
    is_admin boolean default false not null,
    created_at timestamp default current_timestamp
);
create index if not exists users_username_index on users(username);

create table if not exists onchain_addresses (
    id serial primary key,
    user_id integer not null,
    address varchar(255) not null,
    proof text not null,
    created_at timestamp default current_timestamp,
    foreign key (user_id) references users(id)
);
create index if not exists onchain_addresses_address_index on onchain_addresses(address);

create table if not exists direct_messages (
    id serial primary key,
    sender_id integer not null,
    receiver_id integer not null,
    message text not null,
    created_at timestamp default current_timestamp,
    foreign key (sender_id) references users(id),
    foreign key (receiver_id) references users(id)
);
create index if not exists direct_messages_sender_receiver_index on direct_messages(sender_id, receiver_id);

create table if not exists posts (
    id serial primary key,
    user_id integer not null,
    root_id integer,
    parent_id integer,
    title varchar(255),
    link varchar(255),
    content varchar(1024),
    created_at timestamp default current_timestamp not null,
    foreign key (user_id) references users(id),
    foreign key (root_id) references posts(id),
    foreign key (parent_id) references posts(id)
);

create table if not exists upvotes (
    post_id integer not null,
    user_id integer not null,
    foreign key (post_id) references posts(id),
    foreign key (user_id) references users(id),
    constraint UK_post_user UNIQUE(post_id, user_id)   
);