import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

import 'fetchUsers.graphql.dart';

class UserListScreen extends HookWidget {
  const UserListScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final query = useQuery$FetchUsers();
    final result = query.result;

    if (result.hasException) {
      return Scaffold(
        appBar: AppBar(
          title: const Text('User List'),
        ),
        body: Center(
          child: Text(result.exception.toString()),
        ),
      );
    }

    if (result.isLoading) {
      return Scaffold(
        appBar: AppBar(
          title: const Text('User List'),
        ),
        body: const Center(
          child: CircularProgressIndicator(),
        ),
      );
    }

    final data = result.data;

    if (data == null) {
      return Scaffold(
        appBar: AppBar(
          title: const Text('User List'),
        ),
        body: const Center(
          child: Text('No data found'),
        ),
      );
    }

    List<Query$FetchUsers$users> users = result.parserFn(data).users;

    return Scaffold(
      appBar: AppBar(
        title: Text('Users',
            style: Theme.of(context).textTheme.titleLarge?.copyWith(
                  color: Theme.of(context).colorScheme.onPrimary,
                  fontWeight: FontWeight.bold,
                  letterSpacing: 0.8,
                )),
        backgroundColor: Theme.of(context).primaryColor,
      ),
      body: ListView.builder(
        itemCount: users.length,
        itemBuilder: (context, index) {
          final user = users[index];
          return Padding(
            padding: const EdgeInsets.all(8.0),
            child: Card(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(user.name,
                        style: const TextStyle(fontWeight: FontWeight.bold)),
                    Text(user.email,
                        style: const TextStyle(fontWeight: FontWeight.bold)),
                    const SizedBox(height: 8),
                    Text('Skill Level: ${user.skillLevel}'),
                    Text('Climbing Style: ${user.preferredClimbingStyle}'),
                    Text('Gym: ${user.preferredGym}'),
                  ],
                ),
              ),
            ),
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        backgroundColor: Theme.of(context).primaryColor,
        onPressed: () {
          Navigator.pushNamed(context, '/add_user');
        },
        child: const Icon(Icons.add, color: Colors.white),
      ),
    );
  }
}
