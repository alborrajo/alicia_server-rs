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