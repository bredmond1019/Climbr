import 'package:client/screens/find_partner_screen/find_partner_screen.dart';
import 'package:flutter/material.dart';

class HomeScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: Text('Home'),
          backgroundColor: Theme.of(context).primaryColor,
        ),
        body: Center(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Padding(
                padding: const EdgeInsets.all(16.0),
                child: Text(
                  'Welcome to Climbr!',
                  style: TextStyle(
                    fontSize: 24,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
              SizedBox(height: 16),
              SectionCard(
                title: 'Find a Gym',
                color: Colors.blue,
                onTap: () {
                  // TODO: Implement navigation to Find a Gym page
                },
              ),
              SizedBox(height: 16),
              SectionCard(
                title: 'Go to Profile Page',
                color: Colors.green,
                onTap: () {
                  Navigator.pushNamed(context, '/profile');
                },
              ),
              SizedBox(height: 16),
              SectionCard(
                title: 'Look for a Climbing Partner',
                color: Colors.orange,
                onTap: () {
                  Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (context) => const FindPartnerScreen(),
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

  const SectionCard({
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
        margin: EdgeInsets.symmetric(horizontal: 16),
        decoration: BoxDecoration(
          color: color,
          borderRadius: BorderRadius.circular(8),
        ),
        child: Center(
          child: Text(
            title,
            style: TextStyle(
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
