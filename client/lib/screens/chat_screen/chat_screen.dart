import 'dart:convert';

import 'package:client/models/user.dart';
import 'package:client/providers/current_user_provider.dart';
import 'package:client/services/websocket_service.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class ChatScreen extends StatefulWidget {
  final User receiver;
  final int? conversationID;

  const ChatScreen({super.key, required this.receiver, this.conversationID});

  @override
  ChatScreenState createState() => ChatScreenState();
}

class InitialConnectionData {
  String sender_id;
  String receiver_id;
  String conversation_id;

  InitialConnectionData(this.sender_id, this.receiver_id, this.conversation_id);
}

class ChatScreenState extends State<ChatScreen> {
  late WebSocketService webSocketService;
  late CurrentUserProvider userProvider;
  final TextEditingController _controller = TextEditingController();
  late InitialConnectionData initialConnectionData;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    userProvider = Provider.of<CurrentUserProvider>(context, listen: false);
    webSocketService = Provider.of<WebSocketService>(context, listen: false);

    initialConnectionData = InitialConnectionData(
      userProvider.user?.id.toString() ?? '1',
      widget.receiver.id.toString(),
      webSocketService.conversationId.toString(),
    );
    webSocketService.connect(
      'ws://localhost:3000/ws/',
      initialConnectionData,
    );
  }

  // void setConversationID(int id) {
  //   setState(() {
  //     conversationID = id;
  //   });
  // }

  @override
  void dispose() {
    webSocketService.disconnect();
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Chat with ${widget.receiver.name}'),
      ),
      body: Column(
        children: [
          Expanded(
            child: Consumer<WebSocketService>(
              builder: (context, webSocketService, child) {
                return ListView.builder(
                  itemCount: webSocketService.messages.length,
                  itemBuilder: (context, index) {
                    return ListTile(
                      title: Text(webSocketService.messages[index]),
                    );
                  },
                );
              },
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _controller,
                    decoration: const InputDecoration(
                      hintText: 'Enter message',
                    ),
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.send),
                  onPressed: _sendMessage,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  void _sendMessage() {
    final text = _controller.text;
    if (text.isNotEmpty) {
      final conversationId = webSocketService.conversationId;
      if (conversationId != null) {
        webSocketService.send(jsonEncode({
          'conversation_id': conversationId,
          'content': text,
          'sender_id': userProvider.user?.id ?? 1,
        }));
        _controller.clear();
      } else {
        // Handle case where conversation ID is not yet available
      }
    }
  }

  Map<String, dynamic> parseJson(String json) {
    return jsonDecode(json);
  }
}
