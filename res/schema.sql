CREATE TABLE accounts (
    member_no INTEGER PRIMARY KEY NOT NULL,
    login_id TEXT NOT NULL UNIQUE,
    auth_key TEXT
);

CREATE TABLE characters (
    member_no INTEGER NOT NULL,
    character_id INTEGER PRIMARY KEY NOT NULL,
    mount_uid INTEGER NOT NULL UNIQUE,
    nickname TEXT NOT NULL UNIQUE, -- TODO: Check length limit

    -- Parts
    char_id SMALLINT NOT NULL,
    mouth_serial_id SMALLINT NOT NULL,
    face_serial_id SMALLINT NOT NULL,
    parts_val0 SMALLINT NOT NULL,

    -- Appearance
    appearance_val0 SMALLINT NOT NULL,
    head_size SMALLINT NOT NULL,
    height SMALLINT NOT NULL,
    thigh_volume SMALLINT NOT NULL,
    leg_volume SMALLINT NOT NULL,
    appearance_val1 SMALLINT NOT NULL,

    create_character_unk0 INTEGER NOT NULL,

    CONSTRAINT fk_member_no FOREIGN KEY (member_no) REFERENCES accounts(member_no)
);

CREATE TABLE horses (
    character_id INTEGER NOT NULL,

    uid INTEGER PRIMARY KEY NOT NULL,
    tid INTEGER NOT NULL,
    name TEXT NOT NULL,

    -- Parts
    skin_id SMALLINT NOT NULL,
    mane_id SMALLINT NOT NULL,
    tail_id SMALLINT NOT NULL,
    face_id SMALLINT NOT NULL,

    -- Appearance
    scale SMALLINT NOT NULL,
    leg_length SMALLINT NOT NULL,
    leg_volume SMALLINT NOT NULL,
    body_length SMALLINT NOT NULL,
    body_volume SMALLINT NOT NULL,

    -- Stats
    agility INTEGER NOT NULL,
    control INTEGER NOT NULL,
    speed INTEGER NOT NULL,
    strength INTEGER NOT NULL,
    spirit INTEGER NOT NULL,

    rating INTEGER NOT NULL,
    class SMALLINT NOT NULL,
    class_progress SMALLINT NOT NULL,
    grade SMALLINT NOT NULL,
    growth_points SMALLINT NOT NULL,

    -- Vals0
    stamina SMALLINT NOT NULL,
    attractiveness SMALLINT NOT NULL,
    hunger SMALLINT NOT NULL,
    vals0_val0 SMALLINT NOT NULL,
    vals0_val1 SMALLINT NOT NULL,
    vals0_val2 SMALLINT NOT NULL,
    vals0_val3 SMALLINT NOT NULL,
    vals0_val4 SMALLINT NOT NULL,
    vals0_val5 SMALLINT NOT NULL,
    vals0_val6 SMALLINT NOT NULL,
    vals0_val7 SMALLINT NOT NULL,
    vals0_val8 SMALLINT NOT NULL,
    vals0_val9 SMALLINT NOT NULL,
    vals0_val10 SMALLINT NOT NULL,

    -- Vals1
    vals1_val0 SMALLINT NOT NULL,
    vals1_val1 INTEGER NOT NULL,
    date_of_birth INTEGER NOT NULL,
    vals1_val3 SMALLINT NOT NULL,
    vals1_val4 SMALLINT NOT NULL,
    class_progression INTEGER NOT NULL,
    vals1_val5 INTEGER NOT NULL,
    potential_level SMALLINT NOT NULL,
    has_potential SMALLINT NOT NULL,
    potential_value SMALLINT NOT NULL,
    vals1_val9 SMALLINT NOT NULL,
    luck SMALLINT NOT NULL,
    has_luck SMALLINT NOT NULL,
    vals1_val12 SMALLINT NOT NULL,
    fatigue SMALLINT NOT NULL,
    vals1_val14 SMALLINT NOT NULL,
    emblem SMALLINT NOT NULL,

    -- Mastery
    spur_magic_count INTEGER NOT NULL,
    jump_count INTEGER NOT NULL,
    sliding_time INTEGER NOT NULL,
    gliding_distance INTEGER NOT NULL,

    -- Remaining
    val16 INTEGER NOT NULL,
    val17 INTEGER NOT NULL,

    CONSTRAINT fk_character_id FOREIGN KEY (character_id) REFERENCES characters(character_id)
);
