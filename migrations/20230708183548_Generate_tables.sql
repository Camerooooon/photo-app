-- Initialise table for the images
CREATE TABLE IF NOT EXISTS images (
    file_extension TEXT NOT NUll,
    uploaded BIGINT NOT NULL,
    print_available BOOLEAN NOT NULL,
    url TEXT NOT NULL,
    name TEXT NOT NULL,
    privacy ENUM('Listed', 'Unlisted', 'Unspecified') NOT NULL,
    categories TEXT NOT NULL
);

-- Initalise table for image groups
CREATE TABLE IF NOT EXISTS imagegroups (
    created BIGINT NOT NULL,
    name TEXT NOT NULL,
    privacy ENUM('Listed', 'Unlisted', 'Unspecified') NOT NULL,
    url TEXT NOT NULL
);

-- Initialise table for users
CREATE TABLE IF NOT EXISTS users (
    created BIGINT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    permissions TEXT NOT NULL,
    id INT UNSIGNED NOT NULL AUTO_INCREMENT UNIQUE
);

-- Initialise table for api keys
CREATE TABLE IF NOT EXISTS apikeys(
    created BIGINT NOT NULL,
    owner TEXT NOT NULL,
    secret TEXT NOT NULL,
    permissions TEXT NOT NULL,
    expires BIGINT NOT NULL
);
