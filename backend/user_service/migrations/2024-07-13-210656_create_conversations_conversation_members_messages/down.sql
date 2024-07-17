DROP TRIGGER update_conversations_updated_at ON conversations;
DROP TRIGGER update_conversation_members_updated_at ON conversation_members;
DROP TRIGGER update_messages_updated_at ON messages;
DROP FUNCTION update_updated_at_column;
DROP TABLE messages;
DROP TABLE conversation_members;
DROP TABLE conversations;