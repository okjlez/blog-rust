CREATE FUNCTION is_username_taken(target_username VARCHAR) RETURNS BOOLEAN
AS $$
BEGIN 
	IF EXISTS(SELECT 1 from accounts WHERE username = target_username) = true THEN
		RAISE EXCEPTION 'The username % is already taken', target_username USING ERRCODE = '42P10';
		RETURN TRUE;
	END IF;
	RETURN FALSE;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION is_email_taken(target_email VARCHAR) RETURNS BOOLEAN
AS $$
BEGIN 
	IF EXISTS(SELECT 1 from accounts WHERE email = target_email) = true THEN
		RAISE EXCEPTION 'The email % is already taken', target_email USING ERRCODE = '42P11';
		RETURN TRUE;
	END IF;
	RETURN FALSE;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION create_account(id varchar, username varchar, email varchar, password varchar, rank public."Rank")
RETURNS BOOLEAN
AS $$
BEGIN
	IF is_username_taken(username) THEN
		RETURN FALSE;
	END IF;
	IF is_email_taken(email) THEN
		RETURN FALSE;
	END IF;
	INSERT INTO accounts (id, username, email, password, rank) VALUES(id, username, email, password, rank);
	RETURN TRUE;
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION find_by_id(target_id VARCHAR) 
	RETURNS setof accounts
AS $$
BEGIN 
	PERFORM 1 from accounts WHERE accounts.id = target_id;
	IF NOT FOUND THEN 
		RAISE EXCEPTION 'No account found with id %', target_id USING ERRCODE = '42P12';
	END IF;
	RETURN QUERY SELECT * FROM accounts WHERE accounts.id = target_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION find_by_username(target_username VARCHAR) 
	RETURNS setof accounts
AS $$
BEGIN 
	PERFORM 1 from accounts WHERE accounts.username = target_username;
	IF NOT FOUND THEN 
		RAISE EXCEPTION 'No account found with name %', target_username USING ERRCODE = '42P13';
	END IF;
	RETURN QUERY SELECT * FROM accounts WHERE accounts.username = target_username;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION find_by_email(target_email VARCHAR) 
	RETURNS setof accounts
AS $$
BEGIN 
	PERFORM 1 from accounts WHERE accounts.email = target_email;
	IF NOT FOUND THEN 
		RAISE EXCEPTION 'No account found with email %', target_email USING ERRCODE = '42P14';
	END IF;
	RETURN QUERY SELECT * FROM accounts WHERE accounts.email = target_email;
END;
$$ LANGUAGE plpgsql;
