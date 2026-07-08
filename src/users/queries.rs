pub const IS_EXISTS: &str = r#"
SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)
"#;

pub const GET_ALL: &str = r#"
SELECT * FROM users
"#;

pub const GET_BY_ID: &str = r#"
SELECT * FROM users WHERE id = $1
"#;

pub const GET_BY_EMAIL: &str = r#"
SELECT * FROM users WHERE email = $1
"#;

pub const CREATE: &str = r#"
INSERT INTO users (id, first_name, last_name, username, email, "password", created_at, updated_at)
VALUES
    ($1, $2, $3, $4, $5, $6, NOW(), NOW())
RETURNING
	id,
	first_name,
	last_name,
	username,
	email,
  password,
	created_at,
	updated_at
"#;

pub const UPDATE: &str = r#"
UPDATE users
SET
	first_name = $1,
	last_name = $2,
	username = $3,
	email = $4,
	"password" = $5,
	updated_at = NOW()
WHERE
	id = $6
RETURNING
	id,
	first_name,
	last_name,
	username,
	email,
	password,
	created_at AS "created_at: _",
	updated_at AS "updated_at: _"
"#;

pub const DELETE: &str = r#"
DELETE FROM users
WHERE 
    id = $1
RETURNING
    id
"#;
