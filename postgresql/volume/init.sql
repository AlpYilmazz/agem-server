-- create ACCOUNTS table
CREATE TABLE IF NOT EXISTS ACCOUNTS (
    id SERIAL PRIMARY KEY,
    phone_number VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    lastname VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    hashed_password VARCHAR(100) NOT NULL,
    password_set_ts INT8 NOT NULL
);

-- create ROLES table
CREATE TABLE IF NOT EXISTS ROLES (
    id SERIAL PRIMARY KEY,
    role VARCHAR(100) UNIQUE NOT NULL
);

-- create MONTLY_FEES table
CREATE TABLE IF NOT EXISTS MONTLY_FEES (
    id SERIAL PRIMARY KEY,
    year INT NOT NULL,
    month INT NOT NULL
);

-- create ACCOUNT_ROLES table
CREATE TABLE IF NOT EXISTS ACCOUNT_ROLES (
    account_id INT NOT NULL,
    role_id INT NOT NULL,
    PRIMARY KEY(account_id, role_id)
);

-- create PAYMENTS table
CREATE TABLE IF NOT EXISTS PAYMENTS (
    account_id INT NOT NULL,
    fee_id INT NOT NULL,
    PRIMARY KEY(account_id, fee_id)
);

-- create PRE_PAYMENTS table
CREATE TABLE IF NOT EXISTS PRE_PAYMENTS (
    id SERIAL PRIMARY KEY,
    account_id INT NOT NULL,
    year INT NOT NULL,
    month INT NOT NULL
);