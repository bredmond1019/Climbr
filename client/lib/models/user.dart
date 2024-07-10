class User {
  final String id;
  final String name;
  final String email;
  final String password;
  final String skillLevel;
  final String preferredClimmbingStyle;
  final String preferredGym;

  User(
      {required this.id,
      required this.name,
      required this.email,
      required this.password,
      required this.skillLevel,
      required this.preferredClimmbingStyle,
      required this.preferredGym});
}
