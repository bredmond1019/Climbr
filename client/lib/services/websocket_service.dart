import 'dart:async';
import 'dart:convert';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:flutter/material.dart';

class WebSocketService with ChangeNotifier {
  WebSocketChannel? _channel;
  final StreamController<String> _streamController = StreamController<String>();

  Stream<String> get stream => _streamController.stream;

  void connect(String url) {
    _channel = WebSocketChannel.connect(Uri.parse(url));
    _channel!.stream.listen((message) {
      _streamController.add(message);
      notifyListeners();
    });
  }

  void send(String message, int userId, int conversationId) {
    _channel?.sink.add(jsonEncode({
      'sender_id': userId,
      'content': message,
      'conversation_id': conversationId,
    }));
  }

  void sendTypingEvent() {
    // Implement your typing event logic here
  }

  @override
  void dispose() {
    _channel?.sink.close();
    _streamController.close();
    super.dispose();
  }
}
