CREATE TABLE accounts (
    member_no INTEGER PRIMARY KEY NOT NULL,
    login_id TEXT NOT NULL UNIQUE,
    auth_key TEXT
);

CREATE TABLE characters (
    member_no INTEGER NOT NULL,
    character_id INTEGER PRIMARY KEY NOT NULL,
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

CREATE TABLE horse (
    character_id INTEGER NOT NULL,

    uid BIGINT PRIMARY KEY NOT NULL,
    tid BIGINT NOT NULL,
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
    agility BIGINT NOT NULL,
    control BIGINT NOT NULL,
    speed BIGINT NOT NULL,
    strength BIGINT NOT NULL,
    spirit BIGINT NOT NULL,

    rating BIGINT NOT NULL,
    class SMALLINT NOT NULL,
    class_progress SMALLINT NOT NULL,
    grade SMALLINT NOT NULL,
    growth_points INTEGER NOT NULL,

    -- Vals0
    stamina INTEGER NOT NULL,
    attractiveness INTEGER NOT NULL,
    hunger INTEGER NOT NULL,
    vals0_val0 INTEGER NOT NULL,
    vals0_val1 INTEGER NOT NULL,
    vals0_val2 INTEGER NOT NULL,
    vals0_val3 INTEGER NOT NULL,
    vals0_val4 INTEGER NOT NULL,
    vals0_val5 INTEGER NOT NULL,
    vals0_val6 INTEGER NOT NULL,
    vals0_val7 INTEGER NOT NULL,
    vals0_val8 INTEGER NOT NULL,
    vals0_val9 INTEGER NOT NULL,
    vals0_val10 INTEGER NOT NULL,

    -- Vals1
    vals1_val0 SMALLINT NOT NULL,
    vals1_val1 BIGINT NOT NULL,
    date_of_birth BIGINT NOT NULL,
    vals1_val3 SMALLINT NOT NULL,
    vals1_val4 SMALLINT NOT NULL,
    class_progression BIGINT NOT NULL,
    vals1_val5 BIGINT NOT NULL,
    potential_level SMALLINT NOT NULL,
    has_potential SMALLINT NOT NULL,
    potential_value SMALLINT NOT NULL,
    vals1_val9 SMALLINT NOT NULL,
    luck SMALLINT NOT NULL,
    has_luck SMALLINT NOT NULL,
    vals1_val12 SMALLINT NOT NULL,
    fatigue INTEGER NOT NULL,
    vals1_val14 INTEGER NOT NULL,
    emblem INTEGER NOT NULL,

    -- Mastery
    spur_magic_count BIGINT NOT NULL,
    jump_count BIGINT NOT NULL,
    sliding_time BIGINT NOT NULL,
    gliding_distance BIGINT NOT NULL,

    -- Remaining
    val16 BIGINT NOT NULL,
    val17 BIGINT NOT NULL,

    CONSTRAINT fk_character_id FOREIGN KEY (character_id) REFERENCES characters(character_id)
);
