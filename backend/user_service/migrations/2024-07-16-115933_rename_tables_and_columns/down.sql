-- Rename table conversation_memberships back to conversation_member
ALTER TABLE conversation_memberships RENAME TO conversation_members;

-- Rename table chat_messages back to messages
ALTER TABLE chat_messages RENAME TO messages;

-- Rename column user_id back to sender_id in messages table
ALTER TABLE messages RENAME COLUMN user_id TO sender_id;
