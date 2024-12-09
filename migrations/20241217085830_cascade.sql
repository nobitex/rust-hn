-- Add migration script here
alter table posts
drop constraint posts_root_id_fkey,
add constraint posts_root_id_fkey
   foreign key (root_id)
   references posts(id)
   on delete cascade;

alter table posts
drop constraint posts_parent_id_fkey,
add constraint posts_parent_id_fkey
   foreign key (parent_id)
   references posts(id)
   on delete cascade;

alter table posts
drop constraint posts_user_id_fkey,
add constraint posts_user_id_fkey
   foreign key (user_id)
   references users(id)
   on delete cascade;

alter table upvotes
drop constraint upvotes_post_id_fkey,
add constraint upvotes_post_id_fkey
   foreign key (post_id)
   references posts(id)
   on delete cascade;

alter table upvotes
drop constraint upvotes_user_id_fkey,
add constraint upvotes_user_id_fkey
   foreign key (user_id)
   references users(id)
   on delete cascade;