import 'package:client/models/user.dart';
import 'package:client/providers/current_user_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:http/http.dart' as http;
import 'package:provider/provider.dart';
import 'dart:convert';

import 'package:shared_preferences/shared_preferences.dart';

class LoginPage extends HookWidget {
  const LoginPage({super.key});

  @override
  Widget build(BuildContext context) {
    final emailController = useTextEditingController();
    final passwordController = useTextEditingController();

    Future<void> login() async {
      final String email = emailController.text;
      final String password = passwordController.text;

      final response = await http.post(
        Uri.parse('http://localhost:3000/login'),
        headers: <String, String>{
          'Content-Type': 'application/json; charset=UTF-8',
        },
        body: jsonEncode(<String, String>{
          'email': email,
          'password': password,
        }),
      );

      if (response.statusCode == 200) {
        final Map<String, dynamic> responseBody = jsonDecode(response.body);
        final String token = responseBody['token'];
        final user = responseBody['user'];

        // Store token and user information using shared_preferences
        SharedPreferences prefs = await SharedPreferences.getInstance();
        await prefs.setString('token', token);
        await prefs.setString('user', jsonEncode(user));

        Provider.of<CurrentUserProvider>(context, listen: false).setUser(user);
        print(user);

        print('Login successful');

        // Navigate to the Home Page
        Navigator.pushReplacementNamed(context, '/home');
      } else {
        print('Login failed');
      }
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Login'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: <Widget>[
            TextField(
              controller: emailController,
              decoration: const InputDecoration(labelText: 'Email'),
            ),
            TextField(
              controller: passwordController,
              decoration: const InputDecoration(labelText: 'Password'),
              obscureText: true,
            ),
            const SizedBox(height: 20),
            ElevatedButton(
              onPressed: login,
              child: const Text('Login'),
            ),
          ],
        ),
      ),
    );
  }
}
