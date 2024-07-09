import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'services/graphql_service.dart';
import 'screens/home_screen/home_screen.dart';

void main() {
  runApp(const ClimbrApp());
}

class ClimbrApp extends StatelessWidget {
  const ClimbrApp({super.key});

  @override
  Widget build(BuildContext context) {
    return GraphQLProvider(
      client: GraphQLService.initailizeClient(),
      child: MaterialApp(
        title: 'Climbr',
        theme: ThemeData(
          primarySwatch: Colors.blue,
        ),
        home: const HomeScreen(),
      ),
    );
  }
}
