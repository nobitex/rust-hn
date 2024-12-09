-- Add migration script here
create table if not exists chats (
    id serial primary key,
    sender_id integer not null,
    receiver_id integer not null,
    created_at timestamp default current_timestamp not null,
    foreign key (sender_id) references users(id),
    foreign key (receiver_id) references users(id)
);
create index if not exists chats_sender_receiver_index on chats(sender_id, receiver_id);

drop index if exists direct_messages_sender_receiver_index;
alter table direct_messages drop constraint direct_messages_sender_id_fkey;
alter table direct_messages drop constraint direct_messages_receiver_id_fkey;
alter table direct_messages drop column sender_id;
-- alter table direct_messages drop column receiver_id;

alter table direct_messages add column chat_id integer not null;
alter table direct_messages add foreign key (chat_id) references chats(id);
create index if not exists direct_messages_chat_index on direct_messages(chat_id);
-- alter table direct_messages add column receiver_onchain_address_id integer not null;
-- alter table direct_messages add foreign key (receiver_onchain_address_id) references onchain_addresses(id);

-- make onchain_addresses.address unique
alter table onchain_addresses add constraint onchain_addresses_address_unique unique(address);