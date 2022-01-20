-- Add migration script here

CREATE TABLE appUsers(
    id_user serial primary key,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(50) NOT NULL UNIQUE,
    verified_email BOOLEAN,
    crypted_password VARCHAR(500) NOT NULL,
    admin BOOLEAN,
    public_key VARCHAR(500),
    private_key VARCHAR(500),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    salt VARCHAR NOT NULL
);

CREATE TABLE VPN (
    id_VPN serial primary key,
    ip_interface VARCHAR NOT NULL,
    listening_port INT NOT NULL,
    allowed_ip VARCHAR NOT NULL
);

CREATE TABLE vpn_outlet (
    id_outlet serial primary key,
    name VARCHAR(50) NOT NULL,
    user_ip VARCHAR(20) NOT NULL,
    id_user INT REFERENCES appUsers(id_user),
    id_VPN INT REFERENCES VPN(id_VPN) 
);

CREATE TABLE access (
    id_user INT REFERENCES appUsers(id_user),
    id_VPN INT REFERENCES VPN(id_VPN)
);