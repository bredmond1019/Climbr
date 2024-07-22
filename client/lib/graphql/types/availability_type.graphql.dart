class Input$NewAvailabilityInput {
  factory Input$NewAvailabilityInput({
    required String date,
    required int gymId,
    required String startTime,
    required String endTime,
  }) =>
      Input$NewAvailabilityInput._({
        r'date': date,
        r'gymId': gymId,
        r'startTime': startTime,
        r'endTime': endTime,
      });

  Input$NewAvailabilityInput._(this._$data);

  factory Input$NewAvailabilityInput.fromJson(Map<String, dynamic> data) {
    final result$data = <String, dynamic>{};
    final l$date = data['date'];
    result$data['date'] = (l$date as String);
    final l$gymId = data['gymId'];
    result$data['gymId'] = (l$gymId as int);
    final l$startTime = data['startTime'];
    result$data['startTime'] = (l$startTime as String);
    final l$endTime = data['endTime'];
    result$data['endTime'] = (l$endTime as String);
    return Input$NewAvailabilityInput._(result$data);
  }

  Map<String, dynamic> _$data;

  String get date => (_$data['date'] as String);

  int get gymId => (_$data['gymId'] as int);

  String get startTime => (_$data['startTime'] as String);

  String get endTime => (_$data['endTime'] as String);

  Map<String, dynamic> toJson() {
    final result$data = <String, dynamic>{};
    final l$date = date;
    result$data['date'] = l$date;
    final l$gymId = gymId;
    result$data['gymId'] = l$gymId;
    final l$startTime = startTime;
    result$data['startTime'] = l$startTime;
    final l$endTime = endTime;
    result$data['endTime'] = l$endTime;
    return result$data;
  }

  CopyWith$Input$NewAvailabilityInput<Input$NewAvailabilityInput>
      get copyWith => CopyWith$Input$NewAvailabilityInput(
            this,
            (i) => i,
          );

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (!(other is Input$NewAvailabilityInput) ||
        runtimeType != other.runtimeType) {
      return false;
    }
    final l$date = date;
    final lOther$date = other.date;
    if (l$date != lOther$date) {
      return false;
    }
    final l$gymId = gymId;
    final lOther$gymId = other.gymId;
    if (l$gymId != lOther$gymId) {
      return false;
    }
    final l$startTime = startTime;
    final lOther$startTime = other.startTime;
    if (l$startTime != lOther$startTime) {
      return false;
    }
    final l$endTime = endTime;
    final lOther$endTime = other.endTime;
    if (l$endTime != lOther$endTime) {
      return false;
    }
    return true;
  }

  @override
  int get hashCode {
    final l$date = date;
    final l$gymId = gymId;
    final l$startTime = startTime;
    final l$endTime = endTime;
    return Object.hashAll([
      l$date,
      l$gymId,
      l$startTime,
      l$endTime,
    ]);
  }
}

abstract class CopyWith$Input$NewAvailabilityInput<TRes> {
  factory CopyWith$Input$NewAvailabilityInput(
    Input$NewAvailabilityInput instance,
    TRes Function(Input$NewAvailabilityInput) then,
  ) = _CopyWithImpl$Input$NewAvailabilityInput;

  factory CopyWith$Input$NewAvailabilityInput.stub(TRes res) =
      _CopyWithStubImpl$Input$NewAvailabilityInput;

  TRes call({
    String? date,
    int? gymId,
    String? startTime,
    String? endTime,
  });
}

class _CopyWithImpl$Input$NewAvailabilityInput<TRes>
    implements CopyWith$Input$NewAvailabilityInput<TRes> {
  _CopyWithImpl$Input$NewAvailabilityInput(
    this._instance,
    this._then,
  );

  final Input$NewAvailabilityInput _instance;

  final TRes Function(Input$NewAvailabilityInput) _then;

  static const _undefined = <dynamic, dynamic>{};

  TRes call({
    Object? date = _undefined,
    Object? gymId = _undefined,
    Object? startTime = _undefined,
    Object? endTime = _undefined,
  }) =>
      _then(Input$NewAvailabilityInput._({
        ..._instance._$data,
        if (date != _undefined && date != null) 'date': (date as String),
        if (gymId != _undefined && gymId != null) 'gymId': (gymId as int),
        if (startTime != _undefined && startTime != null)
          'startTime': (startTime as String),
        if (endTime != _undefined && endTime != null)
          'endTime': (endTime as String),
      }));
}

class _CopyWithStubImpl$Input$NewAvailabilityInput<TRes>
    implements CopyWith$Input$NewAvailabilityInput<TRes> {
  _CopyWithStubImpl$Input$NewAvailabilityInput(this._res);

  TRes _res;

  call({
    String? date,
    int? gymId,
    String? startTime,
    String? endTime,
  }) =>
      _res;
}
