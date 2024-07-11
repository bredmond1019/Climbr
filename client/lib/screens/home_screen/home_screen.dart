import 'package:client/providers/current_user_provider.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final userProvider = Provider.of<CurrentUserProvider>(context);
    final user = userProvider.user;

    return Scaffold(
        appBar: AppBar(
          backgroundColor: Theme.of(context).primaryColor,
          title: Text('Climbr',
              style: Theme.of(context).textTheme.titleLarge?.copyWith(
                    color: Theme.of(context).colorScheme.onPrimary,
                    fontWeight: FontWeight.bold,
                    letterSpacing: 0.8,
                  )),
        ),
        body: Center(
            child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Padding(
              padding: EdgeInsets.all(16.0),
              child: Text('Welcome to Climbr!'),
            ),
            Center(
                child: user != null
                    ? Padding(
                        padding: const EdgeInsets.all(16.0),
                        child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: <Widget>[
                              Text('Welcome back, ${user.name}!'),
                              const SizedBox(height: 10),
                              Text('Name: ${user.name}'),
                              Text('Email: ${user.email}'),
                              Text('Skill Level: ${user.skillLevel}'),
                              Text(
                                  'Preferred Climbing Style: ${user.preferredClimbingStyle}'),
                              Text('Preferred Gym: ${user.preferredGym}'),
                            ]))
                    : const Text('Welcome!')),
            Padding(
              padding: const EdgeInsets.all(16.0),
              child: ElevatedButton(
                onPressed: () {
                  Navigator.pushNamed(context, '/user_list');
                },
                child: const Text('User List'),
              ),
            ),
          ],
        )));
  }
}
