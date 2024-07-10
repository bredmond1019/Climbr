import 'package:client/screens/add_user_screen/add_user_screen.dart';
import 'package:client/screens/home_screen/home_screen.dart';
import 'package:client/screens/user_list_screen/user_list_screen.dart';
import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:google_fonts/google_fonts.dart';

import 'services/graphql_service.dart';

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
          primaryColor: const Color(0xFF2E7D32), // Forest Green
          highlightColor: const Color(0xFFFFA000), // Amber
          textTheme: GoogleFonts.openSansTextTheme(
            Theme.of(context).textTheme,
          ),
        ),
        initialRoute: '/',
        routes: {
          '/': (context) => const HomeScreen(),
          '/user_list': (context) => const UserListScreen(),
          '/add_user': (context) => AddUserScreen(),
        },
        debugShowCheckedModeBanner: false,
      ),
    );
  }
}
