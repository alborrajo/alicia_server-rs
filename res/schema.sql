CREATE TABLE accounts {
    member_no INTEGER PRIMARY KEY,
    login_id VARCHAR(20) NOT NULL UNIQUE,
    auth_key VARCHAR(64)
};

CREATE TABLE character {
    member_no INTEGER FOREIGN KEY REFERENCES accounts(member_no),
    character_id INTEGER PRIMARY KEY,
    nickname VARCHAR(64) NOT NULL UNIQUE, -- TODO: Check length limit
    char_id INTEGER NOT NULL,
    mouth_serial_id INTEGER NOT NULL,
    face_serial_id INTEGER NOT NULL,
    parts_val0 INTEGER NOT NULL,
    appearance_val0 INTEGER NOT NULL,
    head_size INTEGER NOT NULL,
    height INTEGER NOT NULL,
    thigh_volume INTEGER NOT NULL,
    leg_volume INTEGER NOT NULL,
    val1 INTEGER NOT NULL,
    create_character_unk0 INTEGER NOT NULL,
};