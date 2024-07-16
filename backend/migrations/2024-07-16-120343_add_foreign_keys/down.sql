-- Remove foreign key constraint from chat_messages to conversations
ALTER TABLE chat_messages
DROP CONSTRAINT fk_chat_messages_conversation;

-- Remove foreign key constraint from chat_messages to users
ALTER TABLE chat_messages
DROP CONSTRAINT fk_chat_messages_user;

-- Remove foreign key constraint from conversation_memberships to users
ALTER TABLE conversation_memberships
DROP CONSTRAINT fk_conversation_memberships_user;
