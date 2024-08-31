A quick doc to think about some stuff regarding design choices.

## Client to server communication

I want to make use of JWT for authentication and authorization. This means that the client will have to send a token with every request to the server. This token will be stored in the local memory (and not storage) of the client. This is a bit more secure than storing it in local storage, but it also means that the token will be lost when the user restarts the client. This means that the user will have to log in again every time the client is restarted. 

## Database structure

Status enum:
```sql
CREATE TYPE userstatus AS ENUM ('Online', 'Away', 'Offline');
```

Table users:
```sql
CREATE TABLE users (
id SERIAL PRIMARY KEY,
username TEXT NOT NULL UNIQUE,
password_hash TEXT NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
last_online TIMESTAMPTZ NOT NULL,
status userstatus NOT NULL,
bio TEXT
)
```

Table messages:
```sql
CREATE TABLE messages (
id SERIAL PRIMARY KEY,
sender_id INT NOT NULL,
receiver_id INT NOT NULL,
message TEXT NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

Table group messages: -> ? Looks good, but maybe we should add a group_id field to the messages table instead?
```sql
CREATE TABLE group_messages (
id SERIAL PRIMARY KEY,
group_id INT NOT NULL,
sender_id INT NOT NULL,
message TEXT NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
)
```



# From SP repository
``` sql
CREATE TABLE IF NOT EXISTS Users(
    id integer PRIMARY KEY autoincrement NOT NULL,
    name blob UNIQUE NOT NULL,
    password blob NOT NULL,
    is_logged_in integer NOT NULL,
    cert_text blob NOT NULL);

CREATE TABLE IF NOT EXISTS PrivateMessages(
    id INTEGER PRIMARY KEY autoincrement NOT NULL,
    tx_name blob NOT NULL,
    rx_name blob NOT NULL, 
    msg blob NOT NULL, 
    timestamp integer NOT NULL,
    FOREIGN KEY (tx_name) REFERENCES Users(name),
    FOREIGN KEY (rx_name) REFERENCES Users(name)
);

CREATE TABLE IF NOT EXISTS PublicMessages(
    id INTEGER PRIMARY KEY autoincrement NOT NULL,
    tx_name blob NOT NULL,
    msg blob NOT NULL,
    timestamp integer NOT NULL,
    FOREIGN KEY (tx_name) REFERENCES Users(name)
);

CREATE TABLE IF NOT EXISTS Workers(id integer PRIMARY KEY autoincrement NOT NULL, pid integer NOT NULL, name blob NOT NULL,
    FOREIGN KEY(name) REFERENCES Users(name));
```


Either choose user:
UserClient -- ServerUser


## JSON Sent from server to client

{"id":2,"username":"abcd","password_hash":"test","created_at":[2024,240,12,44,19,649472000,0,0,0],"last_online":[2024,240,12,44,19,649472000,0,0,0],"status":"Offline","bio":null}

## TODO

- [ ] Update DB tables to include messages
- [ ] Get JWT working
- [ ] Get asymmetric encryption working for user to user messages
- [ ] Research group message encryption