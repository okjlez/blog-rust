CREATE DOMAIN username AS VARCHAR(255) CHECK (value ~* '^[a-zA-Z0-9_]{3,}$'); -- Checks if the name has no symbols except underscore and at least 3 characters.
CREATE DOMAIN password AS VARCHAR(255) CHECK (value ~* '^(?=.*[A-Z])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{8,}$'); -- Checks if the password has at least one upper case and 1 special character
CREATE DOMAIN email AS VARCHAR(254) CHECK (value ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'); -- Checks if it is a valid email.

