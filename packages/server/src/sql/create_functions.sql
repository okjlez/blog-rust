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



