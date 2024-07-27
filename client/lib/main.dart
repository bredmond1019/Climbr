import 'package:client/providers/current_user_provider.dart';
import 'package:client/screens/add_user_screen/add_user_screen.dart';
import 'package:client/screens/home_screen/home_screen.dart';
import 'package:client/screens/profile_screen/profile_screen.dart';
import 'package:client/screens/user_list_screen/user_list_screen.dart';
import 'package:client/services/websocket_service.dart';
import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:provider/provider.dart';
import 'services/graphql_service.dart';
import 'package:flutter_web_plugins/flutter_web_plugins.dart';
import 'package:shared_preferences_web/shared_preferences_web.dart';

import 'package:firebase_core/firebase_core.dart';
import 'firebase_options.dart';

void registerPlugins(Registrar registrar) {
  SharedPreferencesPlugin.registerWith(registrar);
  registrar.registerMessageHandler();
}

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await Firebase.initializeApp(
    options: DefaultFirebaseOptions.currentPlatform,
  );
  registerPlugins(webPluginRegistrar);
  runApp(const ClimbrApp());
}

class ClimbrApp extends StatelessWidget {
  const ClimbrApp({super.key});

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<ValueNotifier<GraphQLClient>>(
      future: GraphQLService.initializeClient(),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const MaterialApp(
            home: Scaffold(
              body: Center(
                child: CircularProgressIndicator(),
              ),
            ),
          );
        }

        if (snapshot.hasError) {
          final error = snapshot.error;
          return MaterialApp(
            home: Scaffold(
              body: Center(
                child: Text(
                    'Error initializing GraphQL client: ${error.toString()}'),
              ),
            ),
          );
        }

        return MultiProvider(
          providers: [
            ChangeNotifierProvider(create: (context) => CurrentUserProvider()),
            ChangeNotifierProvider(create: (context) => WebSocketService())
          ],
          child: GraphQLProvider(
            client: snapshot.data!,
            child: MaterialApp(
              title: 'Climbr',
              theme: ThemeData(
                primaryColor: const Color(0xFF4B986C), // Forest Green
                colorScheme: const ColorScheme.light(
                  primary: Color(0xFF4B986C), // Forest Green
                  secondary: Color(0xFF05668D), // Darker Forest Green
                  surface: Color(0xFFE0E0E0), // Light Grey
                  error: Color(0xFFB00020), // Red
                  onPrimary: Color(0xFFFFFFFF), // White
                  onSecondary: Color(0xFFFFFFFF), // White
                  onSurface: Color(0xFF000000), // Black
                  onError: Color(0xFFFFFFFF), // White
                ),
                highlightColor: const Color(0xFFBC6C25), // Amber
                textTheme: GoogleFonts.openSansTextTheme(
                  Theme.of(context).textTheme,
                ),
              ),
              initialRoute: '/',
              routes: {
                // '/': (context) => const LoginPage(),
                '/': (context) => const HomePage(),
                '/user_list': (context) => const UserListScreen(),
                '/add_user': (context) => const AddUserScreen(),
                '/profile': (context) => const ProfileScreen(),
              },
              debugShowCheckedModeBanner: false,
            ),
          ),
        );
      },
    );
  }
}
