CREATE TABLE table_name (
    id VARCHAR(255) PRIMARY KEY,
    username username NOT NULL,
    email VARCHAR(254) NOT NULL,
    password VARCHAR(255) NOT NULL,
    password_salt VARCHAR(255) NOT NULL,
    rank public."Rank" NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
)