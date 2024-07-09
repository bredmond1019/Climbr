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

    List<Query$FetchUsers$users?> users = result.parserFn(data).users;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Users'),
      ),
      body: ListView.builder(
        itemCount: users.length,
        itemBuilder: (context, index) {
          final user = users[index];
          return ListTile(
            title: Text(user?.name ?? ''),
            subtitle: Text(user?.email ?? ''),
          );
        },
      ),
    );
  }
}
