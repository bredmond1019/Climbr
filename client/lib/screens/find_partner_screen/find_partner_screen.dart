import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:flutter_datetime_picker/flutter_datetime_picker.dart';

class FindPartnerPage extends StatefulWidget {
  @override
  _FindPartnerPageState createState() => _FindPartnerPageState();
}

class _FindPartnerPageState extends State<FindPartnerPage> {
  String _selectedGymId;
  DateTime _startTime;
  DateTime _endTime;
  List<SearchAvailability$Query$Availability> _availabilities = [];

  void _searchAvailability() async {
    final client = GraphQLConfig.getClient();

    final options = QueryOptions(
      document: SEARCH_AVAILABILITY_QUERY_DOCUMENT,
      variables: SearchAvailabilityArguments(
        gymId: _selectedGymId,
        startTime: _startTime.toIso8601String(),
        endTime: _endTime.toIso8601String(),
      ).toJson(),
    );

    final result = await client.query(options);

    if (result.hasException) {
      print(result.exception.toString());
      return;
    }

    final searchResult = SearchAvailability$Query.fromJson(result.data);
    setState(() {
      _availabilities = searchResult.searchAvailability;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Find a Climbing Partner'),
      ),
      body: Padding(
        padding: EdgeInsets.all(16.0),
        child: Column(
          children: [
            // Gym Selection
            DropdownButtonFormField<String>(
              value: _selectedGymId,
              onChanged: (value) {
                setState(() {
                  _selectedGymId = value;
                });
              },
              items: [
                // Replace with dynamic gym options
                DropdownMenuItem(value: 'gym1', child: Text('Gym 1')),
                DropdownMenuItem(value: 'gym2', child: Text('Gym 2')),
              ],
              decoration: InputDecoration(labelText: 'Select Gym'),
            ),
            SizedBox(height: 16.0),
            // DateTime Pickers
            TextButton(
              onPressed: () {
                DatePicker.showDateTimePicker(context, showTitleActions: true,
                    onConfirm: (date) {
                  setState(() {
                    _startTime = date;
                  });
                }, currentTime: DateTime.now(), locale: LocaleType.en);
              },
              child: Text(
                'Select Start Time',
                style: TextStyle(color: Colors.blue),
              ),
            ),
            Text(_startTime != null
                ? _startTime.toString()
                : 'No start time selected'),
            SizedBox(height: 16.0),
            TextButton(
              onPressed: () {
                DatePicker.showDateTimePicker(context, showTitleActions: true,
                    onConfirm: (date) {
                  setState(() {
                    _endTime = date;
                  });
                }, currentTime: DateTime.now(), locale: LocaleType.en);
              },
              child: Text(
                'Select End Time',
                style: TextStyle(color: Colors.blue),
              ),
            ),
            Text(_endTime != null
                ? _endTime.toString()
                : 'No end time selected'),
            SizedBox(height: 16.0),
            // Search Button
            ElevatedButton(
              onPressed: _searchAvailability,
              child: Text('Search'),
            ),
            SizedBox(height: 16.0),
            // Display Results
            Expanded(
              child: ListView.builder(
                itemCount: _availabilities.length,
                itemBuilder: (context, index) {
                  final availability = _availabilities[index];
                  return ListTile(
                    title: Text(availability.user.name),
                    subtitle: Text(
                        'Available from ${availability.startTime} to ${availability.endTime}'),
                    onTap: () {
                      // Implement chat or request session functionality here
                    },
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}
