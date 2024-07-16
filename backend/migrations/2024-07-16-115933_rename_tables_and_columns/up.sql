
-- Rename table conversation_member to conversation_memberships
ALTER TABLE conversation_members RENAME TO conversation_memberships;

-- Rename table messages to chat_messages
ALTER TABLE messages RENAME TO chat_messages;

-- Rename column sender_id to user_id in chat_messages table
ALTER TABLE chat_messages RENAME COLUMN sender_id TO user_id;
