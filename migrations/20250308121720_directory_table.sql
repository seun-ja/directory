create table directory (
    id text primary key,
    name text UNIQUE not null ,
    email text UNIQUE null,
    phone text UNIQUE null,
    address text null,
    state text null,
    local_government text null,
    postal_code text null,
    website text null,
    about text null,
    current_population integer null,
    staff_strength integer null,
    year_established integer null,
    curriculum_offered text null,
    subjects_taught text[],
    government_approved boolean null,
    awards_recognition text[],
    management_board text[],
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
