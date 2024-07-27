import 'package:client/screens/chat_screen/chat_screen.dart';
import 'package:client/screens/user_list_screen/user_list_screen.dart';
import 'package:flutter/material.dart';

class ConversationList extends StatefulWidget {
  const ConversationList({super.key});

  @override
  ConversationListState createState() => ConversationListState();
}

class ConversationListState extends State<ConversationList> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Conversations'),
      ),
      body: ListView.builder(
        itemCount: conversations.length,
        itemBuilder: (context, index) {
          final conversation = conversations[index];
          return ListTile(
            title: Text(conversation.name),
            subtitle: Text(conversation.lastMessage),
            onTap: () {
              // Open chat_screen for the selected conversation
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => ChatScreen(
                    conversationId: conversation.id,
                  ),
                ),
              );
            },
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          // Open new conversation screen
          Navigator.push(
            context,
            MaterialPageRoute(
              builder: (context) => const UserListScreen(),
            ),
          );
        },
        child: const Icon(Icons.message),
      ),
    );
  }
}

class Conversation {
  final String id;
  final String name;
  final String lastMessage;

  Conversation(
      {required this.id, required this.name, required this.lastMessage});
}

List<Conversation> conversations = [
  Conversation(id: '1', name: 'John Doe', lastMessage: 'Hello!'),
  Conversation(id: '2', name: 'Jane Smith', lastMessage: 'How are you?'),
  // Add more conversations here
];
