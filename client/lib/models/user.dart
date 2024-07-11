class User {
  final String name;
  final String email;
  final String password;
  final int skillLevel;
  final String preferredClimbingStyle;
  final String preferredGym;

  User({
    required this.name,
    required this.email,
    required this.password,
    required this.skillLevel,
    required this.preferredClimbingStyle,
    required this.preferredGym,
  });

  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      name: json['name'],
      email: json['email'],
      password: 'empty_string',
      skillLevel: json['skill_level'],
      preferredClimbingStyle: json['preferred_climbing_style'],
      preferredGym: json['preferred_gym'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'name': name,
      'email': email,
      'password': password,
      'skillLevel': skillLevel,
      'preferredClimbingStyle': preferredClimbingStyle,
      'preferredGym': preferredGym,
    };
  }
}

class CurrentUser {
  final int id;
  final String name;
  final String email;
  final int skillLevel;
  final String preferredClimbingStyle;
  final String preferredGym;

  CurrentUser({
    required this.id,
    required this.name,
    required this.email,
    required this.skillLevel,
    required this.preferredClimbingStyle,
    required this.preferredGym,
  });

  factory CurrentUser.fromJson(Map<String, dynamic> json) {
    return CurrentUser(
      id: json['id'],
      name: json['name'],
      email: json['email'],
      skillLevel: json['skill_level'],
      preferredClimbingStyle: json['preferred_climbing_style'],
      preferredGym: json['preferred_gym'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'email': email,
      'skillLevel': skillLevel,
      'preferredClimbingStyle': preferredClimbingStyle,
      'preferredGym': preferredGym,
    };
  }
}
