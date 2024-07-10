class Input$NewUserInput {
  factory Input$NewUserInput({
    required String name,
    required String email,
    required String password,
    required int skillLevel,
    required String preferredClimbingStyle,
    required String preferredGym,
  }) =>
      Input$NewUserInput._({
        r'name': name,
        r'email': email,
        r'password': password,
        r'skillLevel': skillLevel,
        r'preferredClimbingStyle': preferredClimbingStyle,
        r'preferredGym': preferredGym,
      });

  Input$NewUserInput._(this._$data);

  factory Input$NewUserInput.fromJson(Map<String, dynamic> data) {
    final result$data = <String, dynamic>{};
    final l$name = data['name'];
    result$data['name'] = (l$name as String);
    final l$email = data['email'];
    result$data['email'] = (l$email as String);
    final l$password = data['password'];
    result$data['password'] = (l$password as String);
    final l$skillLevel = data['skillLevel'];
    result$data['skillLevel'] = (l$skillLevel as int);
    final l$preferredClimbingStyle = data['preferredClimbingStyle'];
    result$data['preferredClimbingStyle'] =
        (l$preferredClimbingStyle as String);
    final l$preferredGym = data['preferredGym'];
    result$data['preferredGym'] = (l$preferredGym as String);
    return Input$NewUserInput._(result$data);
  }

  Map<String, dynamic> _$data;

  String get name => (_$data['name'] as String);

  String get email => (_$data['email'] as String);

  String get password => (_$data['password'] as String);

  int get skillLevel => (_$data['skillLevel'] as int);

  String get preferredClimbingStyle =>
      (_$data['preferredClimbingStyle'] as String);

  String get preferredGym => (_$data['preferredGym'] as String);

  Map<String, dynamic> toJson() {
    final result$data = <String, dynamic>{};
    final l$name = name;
    result$data['name'] = l$name;
    final l$email = email;
    result$data['email'] = l$email;
    final l$password = password;
    result$data['password'] = l$password;
    final l$skillLevel = skillLevel;
    result$data['skillLevel'] = l$skillLevel;
    final l$preferredClimbingStyle = preferredClimbingStyle;
    result$data['preferredClimbingStyle'] = l$preferredClimbingStyle;
    final l$preferredGym = preferredGym;
    result$data['preferredGym'] = l$preferredGym;
    return result$data;
  }

  CopyWith$Input$NewUserInput<Input$NewUserInput> get copyWith =>
      CopyWith$Input$NewUserInput(
        this,
        (i) => i,
      );

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (!(other is Input$NewUserInput) || runtimeType != other.runtimeType) {
      return false;
    }
    final l$name = name;
    final lOther$name = other.name;
    if (l$name != lOther$name) {
      return false;
    }
    final l$email = email;
    final lOther$email = other.email;
    if (l$email != lOther$email) {
      return false;
    }
    final l$password = password;
    final lOther$password = other.password;
    if (l$password != lOther$password) {
      return false;
    }
    final l$skillLevel = skillLevel;
    final lOther$skillLevel = other.skillLevel;
    if (l$skillLevel != lOther$skillLevel) {
      return false;
    }
    final l$preferredClimbingStyle = preferredClimbingStyle;
    final lOther$preferredClimbingStyle = other.preferredClimbingStyle;
    if (l$preferredClimbingStyle != lOther$preferredClimbingStyle) {
      return false;
    }
    final l$preferredGym = preferredGym;
    final lOther$preferredGym = other.preferredGym;
    if (l$preferredGym != lOther$preferredGym) {
      return false;
    }
    return true;
  }

  @override
  int get hashCode {
    final l$name = name;
    final l$email = email;
    final l$password = password;
    final l$skillLevel = skillLevel;
    final l$preferredClimbingStyle = preferredClimbingStyle;
    final l$preferredGym = preferredGym;
    return Object.hashAll([
      l$name,
      l$email,
      l$password,
      l$skillLevel,
      l$preferredClimbingStyle,
      l$preferredGym,
    ]);
  }
}

abstract class CopyWith$Input$NewUserInput<TRes> {
  factory CopyWith$Input$NewUserInput(
    Input$NewUserInput instance,
    TRes Function(Input$NewUserInput) then,
  ) = _CopyWithImpl$Input$NewUserInput;

  factory CopyWith$Input$NewUserInput.stub(TRes res) =
      _CopyWithStubImpl$Input$NewUserInput;

  TRes call({
    String? name,
    String? email,
    String? password,
    int? skillLevel,
    String? preferredClimbingStyle,
    String? preferredGym,
  });
}

class _CopyWithImpl$Input$NewUserInput<TRes>
    implements CopyWith$Input$NewUserInput<TRes> {
  _CopyWithImpl$Input$NewUserInput(
    this._instance,
    this._then,
  );

  final Input$NewUserInput _instance;

  final TRes Function(Input$NewUserInput) _then;

  static const _undefined = <dynamic, dynamic>{};

  TRes call({
    Object? name = _undefined,
    Object? email = _undefined,
    Object? password = _undefined,
    Object? skillLevel = _undefined,
    Object? preferredClimbingStyle = _undefined,
    Object? preferredGym = _undefined,
  }) =>
      _then(Input$NewUserInput._({
        ..._instance._$data,
        if (name != _undefined && name != null) 'name': (name as String),
        if (email != _undefined && email != null) 'email': (email as String),
        if (password != _undefined && password != null)
          'password': (password as String),
        if (skillLevel != _undefined && skillLevel != null)
          'skillLevel': (skillLevel as int),
        if (preferredClimbingStyle != _undefined &&
            preferredClimbingStyle != null)
          'preferredClimbingStyle': (preferredClimbingStyle as String),
        if (preferredGym != _undefined && preferredGym != null)
          'preferredGym': (preferredGym as String),
      }));
}

class _CopyWithStubImpl$Input$NewUserInput<TRes>
    implements CopyWith$Input$NewUserInput<TRes> {
  _CopyWithStubImpl$Input$NewUserInput(this._res);

  TRes _res;

  call({
    String? name,
    String? email,
    String? password,
    int? skillLevel,
    String? preferredClimbingStyle,
    String? preferredGym,
  }) =>
      _res;
}

enum Enum$__TypeKind {
  SCALAR,
  OBJECT,
  INTERFACE,
  UNION,
  ENUM,
  INPUT_OBJECT,
  LIST,
  NON_NULL,
  $unknown;

  factory Enum$__TypeKind.fromJson(String value) =>
      fromJson$Enum$__TypeKind(value);

  String toJson() => toJson$Enum$__TypeKind(this);
}

String toJson$Enum$__TypeKind(Enum$__TypeKind e) {
  switch (e) {
    case Enum$__TypeKind.SCALAR:
      return r'SCALAR';
    case Enum$__TypeKind.OBJECT:
      return r'OBJECT';
    case Enum$__TypeKind.INTERFACE:
      return r'INTERFACE';
    case Enum$__TypeKind.UNION:
      return r'UNION';
    case Enum$__TypeKind.ENUM:
      return r'ENUM';
    case Enum$__TypeKind.INPUT_OBJECT:
      return r'INPUT_OBJECT';
    case Enum$__TypeKind.LIST:
      return r'LIST';
    case Enum$__TypeKind.NON_NULL:
      return r'NON_NULL';
    case Enum$__TypeKind.$unknown:
      return r'$unknown';
  }
}

Enum$__TypeKind fromJson$Enum$__TypeKind(String value) {
  switch (value) {
    case r'SCALAR':
      return Enum$__TypeKind.SCALAR;
    case r'OBJECT':
      return Enum$__TypeKind.OBJECT;
    case r'INTERFACE':
      return Enum$__TypeKind.INTERFACE;
    case r'UNION':
      return Enum$__TypeKind.UNION;
    case r'ENUM':
      return Enum$__TypeKind.ENUM;
    case r'INPUT_OBJECT':
      return Enum$__TypeKind.INPUT_OBJECT;
    case r'LIST':
      return Enum$__TypeKind.LIST;
    case r'NON_NULL':
      return Enum$__TypeKind.NON_NULL;
    default:
      return Enum$__TypeKind.$unknown;
  }
}

enum Enum$__DirectiveLocation {
  QUERY,
  MUTATION,
  SUBSCRIPTION,
  FIELD,
  FRAGMENT_DEFINITION,
  FRAGMENT_SPREAD,
  INLINE_FRAGMENT,
  VARIABLE_DEFINITION,
  SCHEMA,
  SCALAR,
  OBJECT,
  FIELD_DEFINITION,
  ARGUMENT_DEFINITION,
  INTERFACE,
  UNION,
  ENUM,
  ENUM_VALUE,
  INPUT_OBJECT,
  INPUT_FIELD_DEFINITION,
  $unknown;

  factory Enum$__DirectiveLocation.fromJson(String value) =>
      fromJson$Enum$__DirectiveLocation(value);

  String toJson() => toJson$Enum$__DirectiveLocation(this);
}

String toJson$Enum$__DirectiveLocation(Enum$__DirectiveLocation e) {
  switch (e) {
    case Enum$__DirectiveLocation.QUERY:
      return r'QUERY';
    case Enum$__DirectiveLocation.MUTATION:
      return r'MUTATION';
    case Enum$__DirectiveLocation.SUBSCRIPTION:
      return r'SUBSCRIPTION';
    case Enum$__DirectiveLocation.FIELD:
      return r'FIELD';
    case Enum$__DirectiveLocation.FRAGMENT_DEFINITION:
      return r'FRAGMENT_DEFINITION';
    case Enum$__DirectiveLocation.FRAGMENT_SPREAD:
      return r'FRAGMENT_SPREAD';
    case Enum$__DirectiveLocation.INLINE_FRAGMENT:
      return r'INLINE_FRAGMENT';
    case Enum$__DirectiveLocation.VARIABLE_DEFINITION:
      return r'VARIABLE_DEFINITION';
    case Enum$__DirectiveLocation.SCHEMA:
      return r'SCHEMA';
    case Enum$__DirectiveLocation.SCALAR:
      return r'SCALAR';
    case Enum$__DirectiveLocation.OBJECT:
      return r'OBJECT';
    case Enum$__DirectiveLocation.FIELD_DEFINITION:
      return r'FIELD_DEFINITION';
    case Enum$__DirectiveLocation.ARGUMENT_DEFINITION:
      return r'ARGUMENT_DEFINITION';
    case Enum$__DirectiveLocation.INTERFACE:
      return r'INTERFACE';
    case Enum$__DirectiveLocation.UNION:
      return r'UNION';
    case Enum$__DirectiveLocation.ENUM:
      return r'ENUM';
    case Enum$__DirectiveLocation.ENUM_VALUE:
      return r'ENUM_VALUE';
    case Enum$__DirectiveLocation.INPUT_OBJECT:
      return r'INPUT_OBJECT';
    case Enum$__DirectiveLocation.INPUT_FIELD_DEFINITION:
      return r'INPUT_FIELD_DEFINITION';
    case Enum$__DirectiveLocation.$unknown:
      return r'$unknown';
  }
}

Enum$__DirectiveLocation fromJson$Enum$__DirectiveLocation(String value) {
  switch (value) {
    case r'QUERY':
      return Enum$__DirectiveLocation.QUERY;
    case r'MUTATION':
      return Enum$__DirectiveLocation.MUTATION;
    case r'SUBSCRIPTION':
      return Enum$__DirectiveLocation.SUBSCRIPTION;
    case r'FIELD':
      return Enum$__DirectiveLocation.FIELD;
    case r'FRAGMENT_DEFINITION':
      return Enum$__DirectiveLocation.FRAGMENT_DEFINITION;
    case r'FRAGMENT_SPREAD':
      return Enum$__DirectiveLocation.FRAGMENT_SPREAD;
    case r'INLINE_FRAGMENT':
      return Enum$__DirectiveLocation.INLINE_FRAGMENT;
    case r'VARIABLE_DEFINITION':
      return Enum$__DirectiveLocation.VARIABLE_DEFINITION;
    case r'SCHEMA':
      return Enum$__DirectiveLocation.SCHEMA;
    case r'SCALAR':
      return Enum$__DirectiveLocation.SCALAR;
    case r'OBJECT':
      return Enum$__DirectiveLocation.OBJECT;
    case r'FIELD_DEFINITION':
      return Enum$__DirectiveLocation.FIELD_DEFINITION;
    case r'ARGUMENT_DEFINITION':
      return Enum$__DirectiveLocation.ARGUMENT_DEFINITION;
    case r'INTERFACE':
      return Enum$__DirectiveLocation.INTERFACE;
    case r'UNION':
      return Enum$__DirectiveLocation.UNION;
    case r'ENUM':
      return Enum$__DirectiveLocation.ENUM;
    case r'ENUM_VALUE':
      return Enum$__DirectiveLocation.ENUM_VALUE;
    case r'INPUT_OBJECT':
      return Enum$__DirectiveLocation.INPUT_OBJECT;
    case r'INPUT_FIELD_DEFINITION':
      return Enum$__DirectiveLocation.INPUT_FIELD_DEFINITION;
    default:
      return Enum$__DirectiveLocation.$unknown;
  }
}

const possibleTypesMap = <String, Set<String>>{};
