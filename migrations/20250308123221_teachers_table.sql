create table teachers (
    id UUID primary key,
    trcn text null,
    name text not null,
    bio text null,
    email text UNIQUE not null,
    phone text UNIQUE not null,
    state text null,
    local_government text null,
    qualifications text [],
    experience jsonb [],
    speciality text null,
    status text null,
    created_at timestamp not null default now (),
    updated_at timestamp not null default now ()
);
