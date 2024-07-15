CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    account_number BIGINT NOT NULL,
    balance DECIMAL(20, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE OR REPLACE FUNCTION set_random_10_digit_integer() RETURNS TRIGGER AS $$
BEGIN
    NEW.account_number := (FLOOR(random() * 9000000000) + 1000000000)::bigint;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER before_insert_set_account_number
BEFORE INSERT ON accounts
FOR EACH ROW
EXECUTE FUNCTION set_random_10_digit_integer();

CREATE TRIGGER update_modified_time BEFORE UPDATE ON accounts FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();