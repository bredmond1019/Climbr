class Availability {
  final String id;
  final String gymId;
  final DateTime startTime;
  final DateTime endTime;
  final String userId;

  Availability({
    required this.id,
    required this.gymId,
    required this.startTime,
    required this.endTime,
    required this.userId,
  });

  factory Availability.fromJson(Map<String, dynamic> json) {
    return Availability(
      id: json['id'],
      gymId: json['gymId'],
      startTime: DateTime.parse(json['startTime']),
      endTime: DateTime.parse(json['endTime']),
      userId: json['userId'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'gymId': gymId,
      'startTime': startTime.toIso8601String(),
      'endTime': endTime.toIso8601String(),
      'userId': userId,
    };
  }
}
