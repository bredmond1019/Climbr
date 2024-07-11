import '../../graphql/schema.graphql.dart';
import 'dart:async';
import 'package:flutter/widgets.dart' as widgets;
import 'package:gql/ast.dart';
import 'package:graphql/client.dart' as graphql;
import 'package:graphql_flutter/graphql_flutter.dart' as graphql_flutter;

class Variables$Mutation$CreateUser {
  factory Variables$Mutation$CreateUser({required Input$NewUserInput params}) =>
      Variables$Mutation$CreateUser._({
        r'params': params,
      });

  Variables$Mutation$CreateUser._(this._$data);

  factory Variables$Mutation$CreateUser.fromJson(Map<String, dynamic> data) {
    final result$data = <String, dynamic>{};
    final l$params = data['params'];
    result$data['params'] =
        Input$NewUserInput.fromJson((l$params as Map<String, dynamic>));
    return Variables$Mutation$CreateUser._(result$data);
  }

  Map<String, dynamic> _$data;

  Input$NewUserInput get params => (_$data['params'] as Input$NewUserInput);

  Map<String, dynamic> toJson() {
    final result$data = <String, dynamic>{};
    final l$params = params;
    result$data['params'] = l$params.toJson();
    return result$data;
  }

  CopyWith$Variables$Mutation$CreateUser<Variables$Mutation$CreateUser>
      get copyWith => CopyWith$Variables$Mutation$CreateUser(
            this,
            (i) => i,
          );

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (other is! Variables$Mutation$CreateUser ||
        runtimeType != other.runtimeType) {
      return false;
    }
    final l$params = params;
    final lOther$params = other.params;
    if (l$params != lOther$params) {
      return false;
    }
    return true;
  }

  @override
  int get hashCode {
    final l$params = params;
    return Object.hashAll([l$params]);
  }
}

abstract class CopyWith$Variables$Mutation$CreateUser<TRes> {
  factory CopyWith$Variables$Mutation$CreateUser(
    Variables$Mutation$CreateUser instance,
    TRes Function(Variables$Mutation$CreateUser) then,
  ) = _CopyWithImpl$Variables$Mutation$CreateUser;

  factory CopyWith$Variables$Mutation$CreateUser.stub(TRes res) =
      _CopyWithStubImpl$Variables$Mutation$CreateUser;

  TRes call({Input$NewUserInput? params});
}

class _CopyWithImpl$Variables$Mutation$CreateUser<TRes>
    implements CopyWith$Variables$Mutation$CreateUser<TRes> {
  _CopyWithImpl$Variables$Mutation$CreateUser(
    this._instance,
    this._then,
  );

  final Variables$Mutation$CreateUser _instance;

  final TRes Function(Variables$Mutation$CreateUser) _then;

  static const _undefined = <dynamic, dynamic>{};

  @override
  TRes call({Object? params = _undefined}) =>
      _then(Variables$Mutation$CreateUser._({
        ..._instance._$data,
        if (params != _undefined && params != null)
          'params': (params as Input$NewUserInput),
      }));
}

class _CopyWithStubImpl$Variables$Mutation$CreateUser<TRes>
    implements CopyWith$Variables$Mutation$CreateUser<TRes> {
  _CopyWithStubImpl$Variables$Mutation$CreateUser(this._res);

  final TRes _res;

  @override
  call({Input$NewUserInput? params}) => _res;
}

class Mutation$CreateUser {
  Mutation$CreateUser({
    required this.createUser,
    this.$__typename = 'Mutation',
  });

  factory Mutation$CreateUser.fromJson(Map<String, dynamic> json) {
    final l$createUser = json['createUser'];
    final l$$__typename = json['__typename'];
    return Mutation$CreateUser(
      createUser: Mutation$CreateUser$createUser.fromJson(
          (l$createUser as Map<String, dynamic>)),
      $__typename: (l$$__typename as String),
    );
  }

  final Mutation$CreateUser$createUser createUser;

  final String $__typename;

  Map<String, dynamic> toJson() {
    final resultData = <String, dynamic>{};
    final l$createUser = createUser;
    resultData['createUser'] = l$createUser.toJson();
    final l$$__typename = $__typename;
    resultData['__typename'] = l$$__typename;
    return resultData;
  }

  @override
  int get hashCode {
    final l$createUser = createUser;
    final l$$__typename = $__typename;
    return Object.hashAll([
      l$createUser,
      l$$__typename,
    ]);
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (other is! Mutation$CreateUser || runtimeType != other.runtimeType) {
      return false;
    }
    final l$createUser = createUser;
    final lOther$createUser = other.createUser;
    if (l$createUser != lOther$createUser) {
      return false;
    }
    final l$$__typename = $__typename;
    final lOther$$__typename = other.$__typename;
    if (l$$__typename != lOther$$__typename) {
      return false;
    }
    return true;
  }
}

extension UtilityExtension$Mutation$CreateUser on Mutation$CreateUser {
  CopyWith$Mutation$CreateUser<Mutation$CreateUser> get copyWith =>
      CopyWith$Mutation$CreateUser(
        this,
        (i) => i,
      );
}

abstract class CopyWith$Mutation$CreateUser<TRes> {
  factory CopyWith$Mutation$CreateUser(
    Mutation$CreateUser instance,
    TRes Function(Mutation$CreateUser) then,
  ) = _CopyWithImpl$Mutation$CreateUser;

  factory CopyWith$Mutation$CreateUser.stub(TRes res) =
      _CopyWithStubImpl$Mutation$CreateUser;

  TRes call({
    Mutation$CreateUser$createUser? createUser,
    String? $__typename,
  });
  CopyWith$Mutation$CreateUser$createUser<TRes> get createUser;
}

class _CopyWithImpl$Mutation$CreateUser<TRes>
    implements CopyWith$Mutation$CreateUser<TRes> {
  _CopyWithImpl$Mutation$CreateUser(
    this._instance,
    this._then,
  );

  final Mutation$CreateUser _instance;

  final TRes Function(Mutation$CreateUser) _then;

  static const _undefined = <dynamic, dynamic>{};

  @override
  TRes call({
    Object? createUser = _undefined,
    Object? $__typename = _undefined,
  }) =>
      _then(Mutation$CreateUser(
        createUser: createUser == _undefined || createUser == null
            ? _instance.createUser
            : (createUser as Mutation$CreateUser$createUser),
        $__typename: $__typename == _undefined || $__typename == null
            ? _instance.$__typename
            : ($__typename as String),
      ));

  @override
  CopyWith$Mutation$CreateUser$createUser<TRes> get createUser {
    final local$createUser = _instance.createUser;
    return CopyWith$Mutation$CreateUser$createUser(
        local$createUser, (e) => call(createUser: e));
  }
}

class _CopyWithStubImpl$Mutation$CreateUser<TRes>
    implements CopyWith$Mutation$CreateUser<TRes> {
  _CopyWithStubImpl$Mutation$CreateUser(this._res);

  final TRes _res;

  @override
  call({
    Mutation$CreateUser$createUser? createUser,
    String? $__typename,
  }) =>
      _res;

  @override
  CopyWith$Mutation$CreateUser$createUser<TRes> get createUser =>
      CopyWith$Mutation$CreateUser$createUser.stub(_res);
}

const documentNodeMutationCreateUser = DocumentNode(definitions: [
  OperationDefinitionNode(
    type: OperationType.mutation,
    name: NameNode(value: 'CreateUser'),
    variableDefinitions: [
      VariableDefinitionNode(
        variable: VariableNode(name: NameNode(value: 'params')),
        type: NamedTypeNode(
          name: NameNode(value: 'NewUserInput'),
          isNonNull: true,
        ),
        defaultValue: DefaultValueNode(value: null),
        directives: [],
      )
    ],
    directives: [],
    selectionSet: SelectionSetNode(selections: [
      FieldNode(
        name: NameNode(value: 'createUser'),
        alias: null,
        arguments: [
          ArgumentNode(
            name: NameNode(value: 'params'),
            value: VariableNode(name: NameNode(value: 'params')),
          )
        ],
        directives: [],
        selectionSet: SelectionSetNode(selections: [
          FieldNode(
            name: NameNode(value: 'id'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
          FieldNode(
            name: NameNode(value: 'email'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
          FieldNode(
            name: NameNode(value: 'name'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
          FieldNode(
            name: NameNode(value: 'preferredClimbingStyle'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
          FieldNode(
            name: NameNode(value: 'preferredGym'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
          FieldNode(
            name: NameNode(value: 'skillLevel'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
          FieldNode(
            name: NameNode(value: '__typename'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
        ]),
      ),
      FieldNode(
        name: NameNode(value: '__typename'),
        alias: null,
        arguments: [],
        directives: [],
        selectionSet: null,
      ),
    ]),
  ),
]);
Mutation$CreateUser _parserFn$Mutation$CreateUser(Map<String, dynamic> data) =>
    Mutation$CreateUser.fromJson(data);
typedef OnMutationCompleted$Mutation$CreateUser = FutureOr<void> Function(
  Map<String, dynamic>?,
  Mutation$CreateUser?,
);

class Options$Mutation$CreateUser
    extends graphql.MutationOptions<Mutation$CreateUser> {
  Options$Mutation$CreateUser({
    super.operationName,
    required Variables$Mutation$CreateUser variables,
    super.fetchPolicy,
    super.errorPolicy,
    super.cacheRereadPolicy,
    Object? optimisticResult,
    Mutation$CreateUser? typedOptimisticResult,
    super.context,
    OnMutationCompleted$Mutation$CreateUser? onCompleted,
    super.update,
    super.onError,
  })  : onCompletedWithParsed = onCompleted,
        super(
          variables: variables.toJson(),
          optimisticResult: optimisticResult ?? typedOptimisticResult?.toJson(),
          onCompleted: onCompleted == null
              ? null
              : (data) => onCompleted(
                    data,
                    data == null ? null : _parserFn$Mutation$CreateUser(data),
                  ),
          document: documentNodeMutationCreateUser,
          parserFn: _parserFn$Mutation$CreateUser,
        );

  final OnMutationCompleted$Mutation$CreateUser? onCompletedWithParsed;

  @override
  List<Object?> get properties => [
        ...super.onCompleted == null
            ? super.properties
            : super.properties.where((property) => property != onCompleted),
        onCompletedWithParsed,
      ];
}

class WatchOptions$Mutation$CreateUser
    extends graphql.WatchQueryOptions<Mutation$CreateUser> {
  WatchOptions$Mutation$CreateUser({
    super.operationName,
    required Variables$Mutation$CreateUser variables,
    super.fetchPolicy,
    super.errorPolicy,
    super.cacheRereadPolicy,
    Object? optimisticResult,
    Mutation$CreateUser? typedOptimisticResult,
    super.context,
    super.pollInterval,
    super.eagerlyFetchResults,
    super.carryForwardDataOnException,
    super.fetchResults,
  }) : super(
          variables: variables.toJson(),
          optimisticResult: optimisticResult ?? typedOptimisticResult?.toJson(),
          document: documentNodeMutationCreateUser,
          parserFn: _parserFn$Mutation$CreateUser,
        );
}

extension ClientExtension$Mutation$CreateUser on graphql.GraphQLClient {
  Future<graphql.QueryResult<Mutation$CreateUser>> mutate$CreateUser(
          Options$Mutation$CreateUser options) async =>
      await mutate(options);
  graphql.ObservableQuery<Mutation$CreateUser> watchMutation$CreateUser(
          WatchOptions$Mutation$CreateUser options) =>
      watchMutation(options);
}

class Mutation$CreateUser$HookResult {
  Mutation$CreateUser$HookResult(
    this.runMutation,
    this.result,
  );

  final RunMutation$Mutation$CreateUser runMutation;

  final graphql.QueryResult<Mutation$CreateUser> result;
}

Mutation$CreateUser$HookResult useMutation$CreateUser(
    [WidgetOptions$Mutation$CreateUser? options]) {
  final result = graphql_flutter
      .useMutation(options ?? WidgetOptions$Mutation$CreateUser());
  return Mutation$CreateUser$HookResult(
    (variables, {optimisticResult, typedOptimisticResult}) =>
        result.runMutation(
      variables.toJson(),
      optimisticResult: optimisticResult ?? typedOptimisticResult?.toJson(),
    ),
    result.result,
  );
}

graphql.ObservableQuery<Mutation$CreateUser> useWatchMutation$CreateUser(
        WatchOptions$Mutation$CreateUser options) =>
    graphql_flutter.useWatchMutation(options);

class WidgetOptions$Mutation$CreateUser
    extends graphql.MutationOptions<Mutation$CreateUser> {
  WidgetOptions$Mutation$CreateUser({
    super.operationName,
    super.fetchPolicy,
    super.errorPolicy,
    super.cacheRereadPolicy,
    Object? optimisticResult,
    Mutation$CreateUser? typedOptimisticResult,
    super.context,
    OnMutationCompleted$Mutation$CreateUser? onCompleted,
    super.update,
    super.onError,
  })  : onCompletedWithParsed = onCompleted,
        super(
          optimisticResult: optimisticResult ?? typedOptimisticResult?.toJson(),
          onCompleted: onCompleted == null
              ? null
              : (data) => onCompleted(
                    data,
                    data == null ? null : _parserFn$Mutation$CreateUser(data),
                  ),
          document: documentNodeMutationCreateUser,
          parserFn: _parserFn$Mutation$CreateUser,
        );

  final OnMutationCompleted$Mutation$CreateUser? onCompletedWithParsed;

  @override
  List<Object?> get properties => [
        ...super.onCompleted == null
            ? super.properties
            : super.properties.where((property) => property != onCompleted),
        onCompletedWithParsed,
      ];
}

typedef RunMutation$Mutation$CreateUser
    = graphql.MultiSourceResult<Mutation$CreateUser> Function(
  Variables$Mutation$CreateUser, {
  Object? optimisticResult,
  Mutation$CreateUser? typedOptimisticResult,
});
typedef Builder$Mutation$CreateUser = widgets.Widget Function(
  RunMutation$Mutation$CreateUser,
  graphql.QueryResult<Mutation$CreateUser>?,
);

class Mutation$CreateUser$Widget
    extends graphql_flutter.Mutation<Mutation$CreateUser> {
  Mutation$CreateUser$Widget({
    super.key,
    WidgetOptions$Mutation$CreateUser? options,
    required Builder$Mutation$CreateUser builder,
  }) : super(
          options: options ?? WidgetOptions$Mutation$CreateUser(),
          builder: (
            run,
            result,
          ) =>
              builder(
            (
              variables, {
              optimisticResult,
              typedOptimisticResult,
            }) =>
                run(
              variables.toJson(),
              optimisticResult:
                  optimisticResult ?? typedOptimisticResult?.toJson(),
            ),
            result,
          ),
        );
}

class Mutation$CreateUser$createUser {
  Mutation$CreateUser$createUser({
    required this.id,
    required this.email,
    required this.name,
    required this.preferredClimbingStyle,
    required this.preferredGym,
    required this.skillLevel,
    this.$__typename = 'User',
  });

  factory Mutation$CreateUser$createUser.fromJson(Map<String, dynamic> json) {
    final l$id = json['id'];
    final l$email = json['email'];
    final l$name = json['name'];
    final l$preferredClimbingStyle = json['preferredClimbingStyle'];
    final l$preferredGym = json['preferredGym'];
    final l$skillLevel = json['skillLevel'];
    final l$$__typename = json['__typename'];
    return Mutation$CreateUser$createUser(
      id: (l$id as int),
      email: (l$email as String),
      name: (l$name as String),
      preferredClimbingStyle: (l$preferredClimbingStyle as String),
      preferredGym: (l$preferredGym as String),
      skillLevel: (l$skillLevel as int),
      $__typename: (l$$__typename as String),
    );
  }

  final int id;

  final String email;

  final String name;

  final String preferredClimbingStyle;

  final String preferredGym;

  final int skillLevel;

  final String $__typename;

  Map<String, dynamic> toJson() {
    final resultData = <String, dynamic>{};
    final l$id = id;
    resultData['id'] = l$id;
    final l$email = email;
    resultData['email'] = l$email;
    final l$name = name;
    resultData['name'] = l$name;
    final l$preferredClimbingStyle = preferredClimbingStyle;
    resultData['preferredClimbingStyle'] = l$preferredClimbingStyle;
    final l$preferredGym = preferredGym;
    resultData['preferredGym'] = l$preferredGym;
    final l$skillLevel = skillLevel;
    resultData['skillLevel'] = l$skillLevel;
    final l$$__typename = $__typename;
    resultData['__typename'] = l$$__typename;
    return resultData;
  }

  @override
  int get hashCode {
    final l$id = id;
    final l$email = email;
    final l$name = name;
    final l$preferredClimbingStyle = preferredClimbingStyle;
    final l$preferredGym = preferredGym;
    final l$skillLevel = skillLevel;
    final l$$__typename = $__typename;
    return Object.hashAll([
      l$id,
      l$email,
      l$name,
      l$preferredClimbingStyle,
      l$preferredGym,
      l$skillLevel,
      l$$__typename,
    ]);
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (other is! Mutation$CreateUser$createUser ||
        runtimeType != other.runtimeType) {
      return false;
    }
    final l$id = id;
    final lOther$id = other.id;
    if (l$id != lOther$id) {
      return false;
    }
    final l$email = email;
    final lOther$email = other.email;
    if (l$email != lOther$email) {
      return false;
    }
    final l$name = name;
    final lOther$name = other.name;
    if (l$name != lOther$name) {
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
    final l$skillLevel = skillLevel;
    final lOther$skillLevel = other.skillLevel;
    if (l$skillLevel != lOther$skillLevel) {
      return false;
    }
    final l$$__typename = $__typename;
    final lOther$$__typename = other.$__typename;
    if (l$$__typename != lOther$$__typename) {
      return false;
    }
    return true;
  }
}

extension UtilityExtension$Mutation$CreateUser$createUser
    on Mutation$CreateUser$createUser {
  CopyWith$Mutation$CreateUser$createUser<Mutation$CreateUser$createUser>
      get copyWith => CopyWith$Mutation$CreateUser$createUser(
            this,
            (i) => i,
          );
}

abstract class CopyWith$Mutation$CreateUser$createUser<TRes> {
  factory CopyWith$Mutation$CreateUser$createUser(
    Mutation$CreateUser$createUser instance,
    TRes Function(Mutation$CreateUser$createUser) then,
  ) = _CopyWithImpl$Mutation$CreateUser$createUser;

  factory CopyWith$Mutation$CreateUser$createUser.stub(TRes res) =
      _CopyWithStubImpl$Mutation$CreateUser$createUser;

  TRes call({
    int? id,
    String? email,
    String? name,
    String? preferredClimbingStyle,
    String? preferredGym,
    int? skillLevel,
    String? $__typename,
  });
}

class _CopyWithImpl$Mutation$CreateUser$createUser<TRes>
    implements CopyWith$Mutation$CreateUser$createUser<TRes> {
  _CopyWithImpl$Mutation$CreateUser$createUser(
    this._instance,
    this._then,
  );

  final Mutation$CreateUser$createUser _instance;

  final TRes Function(Mutation$CreateUser$createUser) _then;

  static const _undefined = <dynamic, dynamic>{};

  @override
  TRes call({
    Object? id = _undefined,
    Object? email = _undefined,
    Object? name = _undefined,
    Object? preferredClimbingStyle = _undefined,
    Object? preferredGym = _undefined,
    Object? skillLevel = _undefined,
    Object? $__typename = _undefined,
  }) =>
      _then(Mutation$CreateUser$createUser(
        id: id == _undefined || id == null ? _instance.id : (id as int),
        email: email == _undefined || email == null
            ? _instance.email
            : (email as String),
        name: name == _undefined || name == null
            ? _instance.name
            : (name as String),
        preferredClimbingStyle: preferredClimbingStyle == _undefined ||
                preferredClimbingStyle == null
            ? _instance.preferredClimbingStyle
            : (preferredClimbingStyle as String),
        preferredGym: preferredGym == _undefined || preferredGym == null
            ? _instance.preferredGym
            : (preferredGym as String),
        skillLevel: skillLevel == _undefined || skillLevel == null
            ? _instance.skillLevel
            : (skillLevel as int),
        $__typename: $__typename == _undefined || $__typename == null
            ? _instance.$__typename
            : ($__typename as String),
      ));
}

class _CopyWithStubImpl$Mutation$CreateUser$createUser<TRes>
    implements CopyWith$Mutation$CreateUser$createUser<TRes> {
  _CopyWithStubImpl$Mutation$CreateUser$createUser(this._res);

  final TRes _res;

  @override
  call({
    int? id,
    String? email,
    String? name,
    String? preferredClimbingStyle,
    String? preferredGym,
    int? skillLevel,
    String? $__typename,
  }) =>
      _res;
}
