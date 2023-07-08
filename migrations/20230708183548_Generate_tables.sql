-- Initialise table for the images
CREATE TABLE IF NOT EXISTS images (
    uploaded BIGINT NOT NULL,
    print_available BOOLEAN NOT NULL,
    url TEXT NOT NULL, name TEXT NOT NULL,
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
