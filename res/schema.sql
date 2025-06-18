CREATE TABLE accounts (
    member_no INTEGER PRIMARY KEY NOT NULL,
    login_id TEXT NOT NULL UNIQUE,
    auth_key TEXT
);

CREATE TABLE character (
    member_no BIGINT ,
    character_id BIGINT PRIMARY KEY,
    nickname VARCHAR(64) NOT NULL UNIQUE, -- TODO: Check length limit
    char_id BIGINT NOT NULL,
    mouth_serial_id BIGINT NOT NULL,
    face_serial_id BIGINT NOT NULL,
    parts_val0 BIGINT NOT NULL,
    appearance_val0 BIGINT NOT NULL,
    head_size BIGINT NOT NULL,
    height BIGINT NOT NULL,
    thigh_volume BIGINT NOT NULL,
    leg_volume BIGINT NOT NULL,
    val1 BIGINT NOT NULL,
    create_character_unk0 BIGINT NOT NULL,
    CONSTRAINT fk_member_no FOREIGN KEY (member_no) REFERENCES accounts(member_no)
);