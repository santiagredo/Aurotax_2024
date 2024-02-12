-- Add migration script here
CREATE TABLE paypal_orders(
    ord_order_id SERIAL PRIMARY KEY,
    ord_paypal_order_id TEXT NOT NULL UNIQUE,
    ord_order_status TEXT NOT NULL,
    ord_payer_name TEXT NOT NULL,
    ord_payer_email TEXT NOT NULL,
    ord_payer_paypal_id TEXT NOT NULL,
    ord_order_create_time TIMESTAMP WITH TIME ZONE,
    ord_order_update_time TIMESTAMP WITH TIME ZONE
);

CREATE TABLE users(
    usr_user_id SERIAL NOT NULL,
    usr_paypal_order_id INT NOT NULL,
    FOREIGN KEY(usr_paypal_order_id) REFERENCES paypal_orders(ord_order_id),
    usr_email TEXT NOT NULL UNIQUE,
    usr_name TEXT NOT NULL,
    usr_password_hash TEXT NOT NULL DEFAULT ''
);

CREATE TABLE administrators(
    adm_administrator_id SERIAL NOT NULL,
    PRIMARY KEY (adm_administrator_id),
    adm_email TEXT NOT NULL UNIQUE,
    adm_name TEXT NOT NULL,
    adm_password_hash TEXT NOT NULL DEFAULT ''
);

INSERT INTO administrators (adm_email, adm_name, adm_password_hash) 
VALUES ('santiagoagredo@live.com', 'santiago sabogal', '$argon2id$v=19$m=19456,t=2,p=1$16va6CWn65jz388E8hhX/g$HuZvrJi6i1EsWDAtAksP9FdHOlNBTABBdW4WDBycg4E');

CREATE TABLE health_check(
    message TEXT
);

INSERT INTO health_check (message)
VALUES ('Health check: OK')