class Availability {
  final String id;
  final String gymId;
  final String date;
  final String startTime;
  final String endTime;
  final String userId;

  Availability({
    required this.id,
    required this.gymId,
    required this.date,
    required this.startTime,
    required this.endTime,
    required this.userId,
  });

  factory Availability.fromJson(Map<String, dynamic> json) {
    return Availability(
      id: json['id'],
      gymId: json['gym_id'],
      date: json['date'],
      startTime: json['start_time'],
      endTime: json['end_time'],
      userId: json['user_id'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'gym_id': gymId,
      'date': date,
      'start_time': startTime,
      'end_time': endTime,
      'user_id': userId,
    };
  }
}

class NewAvailability {
  final int gymId;
  final String startTime;
  final String endTime;
  final String date;

  NewAvailability({
    required this.gymId,
    required this.startTime,
    required this.endTime,
    required this.date,
  });

  Map<String, dynamic> toJson() {
    return {
      'gymId': gymId,
      'startTime': startTime,
      'endTime': endTime,
      'date': date,
    };
  }
}
