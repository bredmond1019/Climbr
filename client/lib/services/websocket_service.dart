import 'dart:convert';

import 'package:client/screens/chat_screen/chat_screen.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class WebSocketService extends ChangeNotifier {
  WebSocketChannel? _channel;
  int? _conversationId;
  List<String> _messages = [];

  int? get conversationId => _conversationId;
  List<String> get messages => _messages;

  void connect(String url, InitialConnectionData params) {
    final uri = Uri.parse(url).replace(queryParameters: {
      'sender_id': params.sender_id,
      'receiver_id': params.receiver_id,
      'conversation_id': params.conversation_id,
    });
    _channel = WebSocketChannel.connect(uri);
    _channel!.stream.listen((message) {
      _handleMessage(message);
      notifyListeners();
    }, onError: (error) {
      // Handle error
    }, onDone: () {
      // Handle connection closed
    });
  }

  void _handleMessage(String message) {
    final parsedMessage = jsonDecode(message);
    if (parsedMessage['conversation_id'] != null) {
      _conversationId = parsedMessage['conversation_id'];
      // Notify listeners about the new conversation ID
    }
    if (parsedMessage['content'] != null) {
      _messages.add(parsedMessage['content']);
    }
    notifyListeners();
    // Handle other messages
  }

  void send(String message) {
    _channel!.sink.add(message);
  }

  void disconnect() {
    _channel?.sink.close();
    _channel = null;
  }
}
