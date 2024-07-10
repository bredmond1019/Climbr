import 'package:flutter/material.dart';

class AddUserScreen extends StatefulWidget {
  const AddUserScreen({super.key});

  @override
  _AddUserScreenState createState() => _AddUserScreenState();
}

class _AddUserScreenState extends State<AddUserScreen> {
  final _formKey = GlobalKey<FormState>();
  String _email = '';
  String _skillLevel = '';
  String _climbingStyle = '';
  String _gym = '';

  void _submitForm() {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
      // TODO: Save user information
      Navigator.pop(context);
    }
  }

  @override
  Widget build(BuildContext context) {
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
                  _email = value!;
                },
              ),
              TextFormField(
                decoration: const InputDecoration(labelText: 'Skill Level'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a skill level';
                  }
                  return null;
                },
                onSaved: (value) {
                  _skillLevel = value!;
                },
              ),
              TextFormField(
                decoration: const InputDecoration(labelText: 'Climbing Style'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a climbing style';
                  }
                  return null;
                },
                onSaved: (value) {
                  _climbingStyle = value!;
                },
              ),
              TextFormField(
                decoration: const InputDecoration(labelText: 'Gym'),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a gym';
                  }
                  return null;
                },
                onSaved: (value) {
                  _gym = value!;
                },
              ),
              const SizedBox(height: 20),
              ElevatedButton(
                onPressed: _submitForm,
                child: const Text('Add User'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
