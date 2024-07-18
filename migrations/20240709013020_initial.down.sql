ALTER table public.workspaces
    drop constraint workspaces_owner_id_fkey;

alter table public.users
    drop constraint users_ws_id_fk;

ALTER TABLE public.users
    DROP COLUMN ws_id;

delete from
    public.users
WHERE
    id = 0;
drop table if exists public.workspaces cascade;

DROP TABLE if EXISTS messages;
DROP TABLE if EXISTS users;
DROP TABLE if EXISTS chats;
DROP type if EXISTS chat_type;
