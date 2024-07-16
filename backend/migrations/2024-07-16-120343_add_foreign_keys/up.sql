-- Add foreign key constraint from chat_messages to conversations
ALTER TABLE chat_messages
ADD CONSTRAINT fk_chat_messages_conversation
FOREIGN KEY (conversation_id) REFERENCES conversations(id);

-- Add foreign key constraint from chat_messages to users
ALTER TABLE chat_messages
ADD CONSTRAINT fk_chat_messages_user
FOREIGN KEY (user_id) REFERENCES users(id);

-- Add foreign key constraint from conversation_memberships to users
ALTER TABLE conversation_memberships
ADD CONSTRAINT fk_conversation_memberships_user
FOREIGN KEY (user_id) REFERENCES users(id);
