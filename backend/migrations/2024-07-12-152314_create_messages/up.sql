CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    sender_id INT NOT NULL,
    receiver_id INT,
    channel_id INT,
    content TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    CONSTRAINT one_target CHECK (receiver_id IS NOT NULL OR channel_id IS NOT NULL),
    CONSTRAINT only_one_target CHECK (receiver_id IS NULL OR channel_id IS NULL)
);