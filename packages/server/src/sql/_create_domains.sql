CREATE DOMAIN username AS VARCHAR(255) CHECK (value ~* '^[a-zA-Z0-9_]{3,}$'); -- Checks if the name has no symbols except underscore and at least 3 characters.
CREATE DOMAIN email AS VARCHAR(254) CHECK (value ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'); -- Checks if it is a valid email.

