-- Add migration script here
CREATE TABLE subs (
	pincode TEXT NOT NULL,
	age_limit INTEGER NOT NULL,
	reg_token TEXT NOT NULL,
	last_notification_at INTEGER DEFAULT 0 NOT NULL
);

CREATE INDEX idx_pincode
ON subs(pincode);

CREATE INDEX idx_age_limit
ON subs(age_limit);

CREATE UNIQUE INDEX idx_reg_token
ON subs(pincode, age_limit, reg_token);