-- Your SQL goes here
create table public."user" (
    id int8 constraint id primary key,
    created_at int8 default 0 not null,
    updated_at int8 default 0 not null,
    wx_openid varchar not null,
    editor int8 default 0 not null,
    nick_name varchar not null,
    password varchar default 'password' not null,
    mobile varchar not null,
    avatar bytea not null,
    "desc" text not null
);

comment on column public."user".id is '用户id';

comment on column public."user".created_at is '创建时间';

comment on column public."user".updated_at is '最后编辑时间';

comment on column public."user".wx_openid is '微信openid';

comment on column public."user".editor is '最后编辑人id';

comment on column public."user".nick_name is '昵称';

comment on column public."user".password is '密码(md5加密后)';

comment on column public."user".mobile is '手机号';

comment on column public."user".avatar is '头像(列表形式)';

comment on column public."user"."desc" is '用户填写的描述';

create unique index mobile on public."user" (mobile);

comment on index public.mobile is ' 唯一';

create unique index nick_name_password on public."user" (nick_name, password);

comment on index public.nick_name_password is '昵称 密码唯一';

create unique index wx_openid on public."user" (wx_openid);

comment on index public.wx_openid is '用户微信openid 唯一';