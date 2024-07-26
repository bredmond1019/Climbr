import 'package:client/models/gym.dart';
import 'package:client/screens/gym/gym_profile.dart';
import 'package:flutter/material.dart';

class GymListPage extends StatefulWidget {
  const GymListPage({super.key});

  @override
  _GymListPageState createState() => _GymListPageState();
}

class _GymListPageState extends State<GymListPage> {
  final List<Gym> _gyms = [
    Gym(
      name: 'Climb On',
      address: '123 Main St, City',
      climbingTypes: ['Top Rope', 'Sport', 'Bouldering'],
    ),
    Gym(
      name: 'Rock Solid',
      address: '456 Elm St, City',
      climbingTypes: ['Top Rope', 'Bouldering'],
    ),
  ];

  List<Gym> _filteredGyms = [];

  @override
  void initState() {
    super.initState();
    _filteredGyms = _gyms;
  }

  void _filterGyms(String query) {
    setState(() {
      _filteredGyms = _gyms
          .where((gym) => gym.name.toLowerCase().contains(query.toLowerCase()))
          .toList();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Gym List'),
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: TextField(
              onChanged: _filterGyms,
              decoration: const InputDecoration(
                labelText: 'Search',
                prefixIcon: Icon(Icons.search),
              ),
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: _filteredGyms.length,
              itemBuilder: (context, index) {
                final gym = _filteredGyms[index];
                return ListTile(
                  title: Text(
                    gym.name,
                    style: const TextStyle(
                      fontWeight: FontWeight.bold,
                      fontSize: 18,
                    ),
                  ),
                  subtitle: Text(gym.address),
                  trailing: Text(gym.climbingTypes.join(', ')),
                  onTap: () {
                    Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (context) => GymProfilePage(
                            gym: gym,
                          ),
                        ));
                  },
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}
