---- Base app schema


-- User
CREATE TABLE "user" (
                        id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
                        username varchar(128) NOT NULL UNIQUE,
    -- AUTH
                        pwd varchar(256),
                        pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
                        token_salt uuid NOT NULL DEFAULT gen_random_uuid()

);

CREATE TABLE "task" (
                        id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
                        title varchar(128) NOT NULL UNIQUE,
                            done bool NOT NULL DEFAULT false
);