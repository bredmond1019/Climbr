import 'package:client/models/user.dart';
import 'package:client/services/websocket_service.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class ChatScreen extends StatefulWidget {
  final User user;
  const ChatScreen({super.key, required this.user});

  @override
  ChatScreenState createState() => ChatScreenState();
}

class ClientMessage {
  int sender_id;
  String content;
  int conversation_id;

  ClientMessage(this.sender_id, this.content, this.conversation_id);
}

class ChatScreenState extends State<ChatScreen> {
  final TextEditingController _controller = TextEditingController();
  int conversationID = 7;

  void setConversationID(int id) {
    setState(() {
      conversationID = id;
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final webSocketService = Provider.of<WebSocketService>(context);

    return Scaffold(
      appBar: AppBar(
        title: Text('Chat with ${widget.user.name}'),
      ),
      body: Column(
        children: [
          Expanded(
            child: StreamBuilder(
              stream: webSocketService.stream,
              builder: (context, snapshot) {
                if (snapshot.hasData) {
                  // handle your message stream here
                  final messages = snapshot.data;
                  print(messages);
                  return ListView.builder(
                    itemCount: messages?.length,
                    itemBuilder: (context, index) {
                      return ListTile(
                        title: Text('Here is a message'),
                      );
                    },
                  );
                } else {
                  return const Center(child: CircularProgressIndicator());
                }
              },
            ),
          ),
          // StreamBuilder<String>(
          //   stream: webSocketService.typingStream,
          //   builder: (context, snapshot) {
          //     if (snapshot.hasData && messages!.isNotEmpty) {
          //       return Text(messages!);
          //     }
          //     return SizedBox.shrink();
          //   },
          // ),
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
                    // onChanged: (text) {
                    //   webSocketService.sendTypingEvent();
                    // },
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.send),
                  onPressed: () {
                    webSocketService.send(_controller.text, 1, 7);
                    _controller.clear();
                  },
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
