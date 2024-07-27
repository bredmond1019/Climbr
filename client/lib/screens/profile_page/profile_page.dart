import 'package:client/models/user.dart';
import 'package:client/providers/current_user_provider.dart';
import 'package:client/screens/profile_page/edit_profile.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class ProfilePage extends StatelessWidget {
  const ProfilePage({super.key});

  @override
  Widget build(BuildContext context) {
    final CurrentUser currentUser =
        Provider.of<CurrentUserProvider>(context).user;
    return SingleChildScrollView(
      child: Column(
        children: [
          UserProfileHeader(currentUser: currentUser),
          const UserGymList(),
          const UserAvailabilityList(),
          EditProfileButton(currentUser: currentUser),
        ],
      ),
    );
  }
}

class UserProfileHeader extends StatelessWidget {
  final CurrentUser currentUser;

  const UserProfileHeader({super.key, required this.currentUser});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16.0),
      child: const Column(
        children: [
          CircleAvatar(
            radius: 50,
            backgroundImage: AssetImage('assets/user_placeholder.png'),
          ),
          SizedBox(height: 16),
          Text(
            'User Name',
            style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
          ),
          SizedBox(height: 8),
          Text('user@example.com'),
        ],
      ),
    );
  }
}

class UserGymList extends StatelessWidget {
  const UserGymList({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Associated Gyms',
            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 8),
          ListView(
            shrinkWrap: true,
            children: const [
              ListTile(
                title: Text('Gym 1'),
                subtitle: Text('Location 1'),
              ),
              ListTile(
                title: Text('Gym 2'),
                subtitle: Text('Location 2'),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class UserAvailabilityList extends StatelessWidget {
  const UserAvailabilityList({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Availability',
            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 8),
          ListView(
            shrinkWrap: true,
            children: const [
              ListTile(
                title: Text('Monday, 6 PM - 8 PM'),
                subtitle: Text('Gym 1, Top Rope'),
              ),
              ListTile(
                title: Text('Wednesday, 7 PM - 9 PM'),
                subtitle: Text('Gym 2, Bouldering'),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class EditProfileButton extends StatelessWidget {
  final CurrentUser currentUser;

  const EditProfileButton({super.key, required this.currentUser});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: ElevatedButton(
        onPressed: () {
          Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => EditProfilePage(
                  currentUser: currentUser,
                ),
              ));
        },
        child: const Text('Edit Profile'),
      ),
    );
  }
}
