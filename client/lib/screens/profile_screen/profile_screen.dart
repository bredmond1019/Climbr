import 'package:client/providers/current_user_provider.dart';
import 'package:client/screens/find_partner_screen/find_partner_screen.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class ProfileScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final userProvider = Provider.of<CurrentUserProvider>(context);
    final user = userProvider.user;

    return Scaffold(
      appBar: AppBar(
        title: Text('Profile'),
      ),
      body: SingleChildScrollView(
        child: Column(
          children: [
            Container(
              width: double.infinity,
              height: 200,
              color: Colors.grey[300],
              child: Center(
                child: CircleAvatar(
                  radius: 50,
                  backgroundImage: AssetImage('assets/profile_image.png'),
                ),
              ),
            ),
            SizedBox(height: 20),
            Text(
              user.name,
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
              ),
            ),
            SizedBox(height: 10),
            Text(
              'I am a climber',
              style: TextStyle(
                fontSize: 16,
                color: Colors.grey,
              ),
            ),
            SizedBox(height: 20),
            ListTile(
              leading: Icon(Icons.email),
              title: Text(user.email),
            ),
            ListTile(
              leading: Icon(Icons.phone),
              title: Text('123-456-7890'),
            ),
            ListTile(
              leading: Icon(Icons.location_on),
              title: Text('New York, NY'),
            ),
            ElevatedButton(
              onPressed: () {
                Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (context) => const FindPartnerScreen(),
                    ));
              },
              child: const Text('Find Partner'),
            ),
            const SizedBox(height: 10),
            ElevatedButton(
              onPressed: () {
                Navigator.pushNamed(context, '/user_list');
              },
              child: const Text('User List'),
            ),
          ],
        ),
      ),
    );
  }
}
