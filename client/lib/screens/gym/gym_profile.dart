import 'package:client/models/gym.dart';
import 'package:flutter/material.dart';

class GymProfilePage extends StatelessWidget {
  final Gym gym; // Declare a final variable to store the Gym object

  const GymProfilePage({super.key, required this.gym}); // Constructor that accepts a Gym object

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Gym Profile'),
      ),
      body: SingleChildScrollView(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Image section
            Container(
              height: 200, // Adjust the height as needed
              decoration: const BoxDecoration(
                image: DecorationImage(
                  image: AssetImage(
                      'assets/gym_image.jpg'), // Replace with the actual image path
                  fit: BoxFit.cover,
                ),
              ),
            ),
            const SizedBox(height: 16), // Add some spacing

            // Gym details section
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    gym.name, // Access the name property of the Gym object
                    style: const TextStyle(
                      fontSize: 24,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    gym.address, // Access the address property of the Gym object
                    style: const TextStyle(
                      fontSize: 16,
                      color: Colors.grey,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    gym.climbingTypes.join(
                        ', '), // Convert the list of climbing types to a string
                    style: const TextStyle(
                      fontSize: 16,
                      color: Colors.grey,
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 16), // Add some spacing

            // Users section
            const Padding(
              padding: EdgeInsets.symmetric(horizontal: 16),
              child: Text(
                'Users',
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
            const SizedBox(height: 8),
            // Add your user list widget here
          ],
        ),
      ),
    );
  }
}
