import 'package:client/graphql/schema.graphql.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

import './login.graphql.dart';
// import '../services/graphql_service.dart';

class LoginScreen extends HookWidget {
  final _formKey = GlobalKey<FormState>();

  LoginScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final email = useState('');
    final password = useState('');

    final mutation = useMutation$Login();

    void submitForm() {
      if (_formKey.currentState!.validate()) {
        _formKey.currentState!.save();
        final response = mutation.runMutation(Variables$Mutation$Login(
          params: Input$LoginInput(
            email: email.value,
            password: password.value,
          ),
        ));

        print(response);

        // .then((result) {
        //   if (result.data != null) {
        //     final token = result.data!['login']['token'];
        //     GraphQLService.saveToken(token);
        //     Navigator.pushReplacementNamed(context, '/user_list');
        //   } else if (result.hasException) {
        //     ScaffoldMessenger.of(context).showSnackBar(
        //       SnackBar(content: Text(result.exception.toString())),
        //     );
        //   }
        // });
      }
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Login'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Form(
          key: _formKey,
          child: Column(
            children: [
              TextFormField(
                decoration: const InputDecoration(labelText: 'Email'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter an email';
                  }
                  return null;
                },
                onSaved: (value) {
                  email.value = value!;
                },
              ),
              TextFormField(
                decoration: const InputDecoration(labelText: 'Password'),
                obscureText: true,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a password';
                  }
                  return null;
                },
                onSaved: (value) {
                  password.value = value!;
                },
              ),
              const SizedBox(height: 20),
              ElevatedButton(
                onPressed: submitForm,
                child: const Text('Login'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
