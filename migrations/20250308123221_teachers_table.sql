create table teachers (
    id serial primary key,
    trcn integer null,
    name text not null,
    bio text null,
    email text UNIQUE not null,
    phone text UNIQUE not null,
    state text null,
    local_government text null,
    qualifications text [],
    experience jsonb [],
    speciality text not null,
    status text not null,
    created_at timestamp not null default now (),
    updated_at timestamp not null default now ()
);
