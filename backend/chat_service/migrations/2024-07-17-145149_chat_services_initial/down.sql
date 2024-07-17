-- This file should undo anything in `up.sql`
DROP TRIGGER update_conversations_updated_at ON conversations;
DROP TRIGGER update_conversation_members_updated_at ON conversation_memberships;
DROP TRIGGER update_messages_updated_at ON chat_messages;

DROP TABLE chat_messages;
DROP TABLE conversation_memberships;
DROP TABLE conversations;