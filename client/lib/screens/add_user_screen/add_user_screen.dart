import 'package:client/graphql/schema.graphql.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

import 'package:client/screens/add_user_screen/createUser.graphql.dart';

class AddUserScreen extends HookWidget {
  final _formKey = GlobalKey<FormState>();

  AddUserScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final emailController = useTextEditingController();
    final nameController = useTextEditingController();
    final passwordController = useTextEditingController();
    final preferredClimbingStyleController = useTextEditingController();
    final preferredGymController = useTextEditingController();
    final skillLevelController = useTextEditingController();

    final mutation = useMutation$CreateUser();

    void submitForm() {
      if (_formKey.currentState!.validate()) {
        _formKey.currentState!.save();
        mutation.runMutation(Variables$Mutation$CreateUser(
            params: Input$NewUserInput(
          email: emailController.text,
          name: nameController.text,
          password: passwordController.text,
          preferredClimbingStyle: preferredClimbingStyleController.text,
          preferredGym: preferredGymController.text,
          skillLevel: int.tryParse(skillLevelController.text) ?? 0,
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
                controller: emailController,
                decoration: const InputDecoration(labelText: 'Email'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter an email';
                  }
                  return null;
                },
              ),
              TextFormField(
                controller: nameController,
                decoration: const InputDecoration(labelText: 'Name'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a name';
                  }
                  return null;
                },
              ),
              TextFormField(
                controller: passwordController,
                decoration: const InputDecoration(labelText: 'Password'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a password';
                  }
                  return null;
                },
              ),
              TextFormField(
                controller: skillLevelController,
                decoration: const InputDecoration(labelText: 'Skill Level'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a Skill Level (beginner, intermediate, advanced)';
                  }
                  return null;
                },
              ),
              TextFormField(
                controller: preferredClimbingStyleController,
                decoration: const InputDecoration(
                    labelText: 'Preferred Style of Climbing'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a preferred style of climbing';
                  }
                  return null;
                },
              ),
              TextFormField(
                controller: preferredGymController,
                decoration: const InputDecoration(labelText: 'Preferred Gym'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a preferred gym';
                  }
                  return null;
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
