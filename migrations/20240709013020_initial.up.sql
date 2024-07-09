-- Add up migration script here

-- this file is used for postgresql database initializtion
-- create user table
CREATE TABLE IF NOT EXISTS users(
    id bigserial primary key,
    fullname varchar(64) NOT Null,
    email varchar(64) NOT Null,
    -- hashed argon2 password
    password_hash varchar(128) NOT Null,
    created_at timestamptz  default Current_timestamp
);

--create index for users for email
CREATE Unique index if NOT EXISTS email_index On users(email);

-- create chat type : single ,group ,private_channel ,public_channel
CREATE type  chat_type AS ENUM  (
    'single',
    'group',
    'private_channel',
    'public_channel'
);

-- create chat table
CREATE TABLE IF NOT EXISTS chats(
    id bigserial primary key,
    name varchar(128) NOT NULL Unique,
    type chat_type NOT NULL,
    -- user id list
    members bigint [] NOT NULL,
    created_at timestamptz  default Current_timestamp
);

-- create message table
CREATE TABLE IF NOT EXISTS messages(
    id bigserial primary key,
    chat_id bigint NOT Null,
    sender_id bigint NOT Null,
    content text NOT Null,
    images text [],
    created_at timestamptz   default Current_timestamp,
    foreign key (chat_id) references chats(id),
    foreign key (sender_id) references users(id)
);

-- create index for messages for chat_id and created_at order by created_at desc
CREATE index if NOT EXISTS chat_id_created_at_index On messages(chat_id, created_at DESC);

-- create index for messages for sender_id
CREATE index if NOT EXISTS sender_id_index On messages(sender_id);
