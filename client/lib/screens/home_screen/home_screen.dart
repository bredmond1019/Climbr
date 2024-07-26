import 'package:client/screens/gym/gym_list.dart';
import 'package:client/screens/user_list_screen/user_list_screen.dart';
import 'package:flutter/material.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: const Text('Home'),
          backgroundColor: Theme.of(context).primaryColor,
        ),
        body: Center(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              const Padding(
                padding: EdgeInsets.all(16.0),
                child: Text(
                  'Welcome to Climbr!',
                  style: TextStyle(
                    fontSize: 24,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
              const SizedBox(height: 16),
              SectionCard(
                title: 'Find a Gym',
                color: Colors.blue,
                onTap: () {
                  Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (context) => const GymListPage(),
                      ));
                },
              ),
              const SizedBox(height: 16),
              SectionCard(
                title: 'Go to Profile Page',
                color: Colors.green,
                onTap: () {
                  Navigator.pushNamed(context, '/profile');
                },
              ),
              const SizedBox(height: 16),
              SectionCard(
                title: 'Look for a Climbing Partner',
                color: Colors.orange,
                onTap: () {
                  Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (context) => const UserListScreen(),
                      ));
                },
              ),
            ],
          ),
        ));
  }
}

class SectionCard extends StatelessWidget {
  final String title;
  final Color color;
  final VoidCallback onTap;

  const SectionCard({super.key, 
    required this.title,
    required this.color,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        height: 100,
        margin: const EdgeInsets.symmetric(horizontal: 16),
        decoration: BoxDecoration(
          color: color,
          borderRadius: BorderRadius.circular(8),
        ),
        child: Center(
          child: Text(
            title,
            style: const TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
        ),
      ),
    );
  }
}
