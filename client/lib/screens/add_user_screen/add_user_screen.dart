import 'package:client/graphql/schema.graphql.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

import 'package:client/screens/add_user_screen/createUser.graphql.dart';

class AddUserScreen extends HookWidget {
  final _formKey = GlobalKey<FormState>();

  AddUserScreen({super.key});

  @override
  Widget build(BuildContext context) {
    // Use useState to manage the state of each field
    final email = useState('');
    final name = useState('');
    final password = useState('');
    final preferredClimbingStyle = useState('');
    final preferredGym = useState('');
    final skillLevel = useState(0);

    final mutation = useMutation$CreateUser();

    void submitForm() {
      if (_formKey.currentState!.validate()) {
        _formKey.currentState!.save();
        mutation.runMutation(Variables$Mutation$CreateUser(
            params: Input$NewUserInput(
          email: email.value,
          name: name.value,
          password: password.value,
          preferredClimbingStyle: preferredClimbingStyle.value,
          preferredGym: preferredGym.value,
          skillLevel: skillLevel.value,
        )));
      }
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Add User'),
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
                decoration: const InputDecoration(labelText: 'Name'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a name';
                  }
                  return null;
                },
                onSaved: (value) {
                  name.value = value!;
                },
              ),
              TextFormField(
                decoration: const InputDecoration(labelText: 'Password'),
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
              TextFormField(
                decoration: const InputDecoration(labelText: 'Skill Level'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a Skill Level (beginner, intermediate, advanced)';
                  }
                  return null;
                },
                onSaved: (value) {
                  skillLevel.value = int.tryParse(value!) ?? 0;
                },
              ),
              TextFormField(
                decoration: const InputDecoration(
                    labelText: 'Preferred Style of Climbing'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a preferred style of climbing';
                  }
                  return null;
                },
                onSaved: (value) {
                  preferredClimbingStyle.value = value!;
                },
              ),
              TextFormField(
                decoration: const InputDecoration(labelText: 'Preferred Gym'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a preferred gym';
                  }
                  return null;
                },
                onSaved: (value) {
                  preferredGym.value = value!;
                },
              ),
              const SizedBox(height: 20),
              ElevatedButton(
                onPressed: () => submitForm(),
                child: const Text('Add User'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
