import 'dart:async';
import 'package:flutter/widgets.dart' as widgets;
import 'package:gql/ast.dart';
import 'package:graphql/client.dart' as graphql;
import 'package:graphql_flutter/graphql_flutter.dart' as graphql_flutter;

class Variables$Query$searchAvailability {
  factory Variables$Query$searchAvailability({
    required int gymId,
    required String startTime,
    required String endTime,
    required String date,
  }) =>
      Variables$Query$searchAvailability._({
        r'gymId': gymId,
        r'startTime': startTime,
        r'endTime': endTime,
        r'date': date,
      });

  Variables$Query$searchAvailability._(this._$data);

  factory Variables$Query$searchAvailability.fromJson(
      Map<String, dynamic> data) {
    final result$data = <String, dynamic>{};
    final l$gymId = data['gymId'];
    result$data['gymId'] = (l$gymId as int);
    final l$startTime = data['startTime'];
    result$data['startTime'] = (l$startTime as String);
    final l$endTime = data['endTime'];
    result$data['endTime'] = (l$endTime as String);
    final l$date = data['date'];
    result$data['date'] = (l$date as String);
    return Variables$Query$searchAvailability._(result$data);
  }

  Map<String, dynamic> _$data;

  int get gymId => (_$data['gymId'] as int);

  String get startTime => (_$data['startTime'] as String);

  String get endTime => (_$data['endTime'] as String);

  String get date => (_$data['date'] as String);

  Map<String, dynamic> toJson() {
    final result$data = <String, dynamic>{};
    final l$gymId = gymId;
    result$data['gymId'] = l$gymId;
    final l$startTime = startTime;
    result$data['startTime'] = l$startTime;
    final l$endTime = endTime;
    result$data['endTime'] = l$endTime;
    final l$date = date;
    result$data['date'] = l$date;
    return result$data;
  }

  CopyWith$Variables$Query$searchAvailability<
          Variables$Query$searchAvailability>
      get copyWith => CopyWith$Variables$Query$searchAvailability(
            this,
            (i) => i,
          );

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (!(other is Variables$Query$searchAvailability) ||
        runtimeType != other.runtimeType) {
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
    final l$date = date;
    final lOther$date = other.date;
    if (l$date != lOther$date) {
      return false;
    }
    return true;
  }

  @override
  int get hashCode {
    final l$gymId = gymId;
    final l$startTime = startTime;
    final l$endTime = endTime;
    final l$date = date;
    return Object.hashAll([
      l$gymId,
      l$startTime,
      l$endTime,
      l$date,
    ]);
  }
}

abstract class CopyWith$Variables$Query$searchAvailability<TRes> {
  factory CopyWith$Variables$Query$searchAvailability(
    Variables$Query$searchAvailability instance,
    TRes Function(Variables$Query$searchAvailability) then,
  ) = _CopyWithImpl$Variables$Query$searchAvailability;

  factory CopyWith$Variables$Query$searchAvailability.stub(TRes res) =
      _CopyWithStubImpl$Variables$Query$searchAvailability;

  TRes call({
    int? gymId,
    String? startTime,
    String? endTime,
    String? date,
  });
}

class _CopyWithImpl$Variables$Query$searchAvailability<TRes>
    implements CopyWith$Variables$Query$searchAvailability<TRes> {
  _CopyWithImpl$Variables$Query$searchAvailability(
    this._instance,
    this._then,
  );

  final Variables$Query$searchAvailability _instance;

  final TRes Function(Variables$Query$searchAvailability) _then;

  static const _undefined = <dynamic, dynamic>{};

  TRes call({
    Object? gymId = _undefined,
    Object? startTime = _undefined,
    Object? endTime = _undefined,
    Object? date = _undefined,
  }) =>
      _then(Variables$Query$searchAvailability._({
        ..._instance._$data,
        if (gymId != _undefined && gymId != null) 'gymId': (gymId as int),
        if (startTime != _undefined && startTime != null)
          'startTime': (startTime as String),
        if (endTime != _undefined && endTime != null)
          'endTime': (endTime as String),
        if (date != _undefined && date != null) 'date': (date as String),
      }));
}

class _CopyWithStubImpl$Variables$Query$searchAvailability<TRes>
    implements CopyWith$Variables$Query$searchAvailability<TRes> {
  _CopyWithStubImpl$Variables$Query$searchAvailability(this._res);

  TRes _res;

  call({
    int? gymId,
    String? startTime,
    String? endTime,
    String? date,
  }) =>
      _res;
}

class Query$searchAvailability {
  Query$searchAvailability({
    this.availabilities,
    this.$__typename = 'Query',
  });

  factory Query$searchAvailability.fromJson(Map<String, dynamic> json) {
    final l$availabilities = json['availabilities'];
    final l$$__typename = json['__typename'];
    return Query$searchAvailability(
      availabilities: (l$availabilities as List<dynamic>?)
          ?.map((e) => e == null
              ? null
              : Query$searchAvailability$availabilities.fromJson(
                  (e as Map<String, dynamic>)))
          .toList(),
      $__typename: (l$$__typename as String),
    );
  }

  final List<Query$searchAvailability$availabilities?>? availabilities;

  final String $__typename;

  Map<String, dynamic> toJson() {
    final _resultData = <String, dynamic>{};
    final l$availabilities = availabilities;
    _resultData['availabilities'] =
        l$availabilities?.map((e) => e?.toJson()).toList();
    final l$$__typename = $__typename;
    _resultData['__typename'] = l$$__typename;
    return _resultData;
  }

  @override
  int get hashCode {
    final l$availabilities = availabilities;
    final l$$__typename = $__typename;
    return Object.hashAll([
      l$availabilities == null
          ? null
          : Object.hashAll(l$availabilities.map((v) => v)),
      l$$__typename,
    ]);
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (!(other is Query$searchAvailability) ||
        runtimeType != other.runtimeType) {
      return false;
    }
    final l$availabilities = availabilities;
    final lOther$availabilities = other.availabilities;
    if (l$availabilities != null && lOther$availabilities != null) {
      if (l$availabilities.length != lOther$availabilities.length) {
        return false;
      }
      for (int i = 0; i < l$availabilities.length; i++) {
        final l$availabilities$entry = l$availabilities[i];
        final lOther$availabilities$entry = lOther$availabilities[i];
        if (l$availabilities$entry != lOther$availabilities$entry) {
          return false;
        }
      }
    } else if (l$availabilities != lOther$availabilities) {
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

extension UtilityExtension$Query$searchAvailability
    on Query$searchAvailability {
  CopyWith$Query$searchAvailability<Query$searchAvailability> get copyWith =>
      CopyWith$Query$searchAvailability(
        this,
        (i) => i,
      );
}

abstract class CopyWith$Query$searchAvailability<TRes> {
  factory CopyWith$Query$searchAvailability(
    Query$searchAvailability instance,
    TRes Function(Query$searchAvailability) then,
  ) = _CopyWithImpl$Query$searchAvailability;

  factory CopyWith$Query$searchAvailability.stub(TRes res) =
      _CopyWithStubImpl$Query$searchAvailability;

  TRes call({
    List<Query$searchAvailability$availabilities?>? availabilities,
    String? $__typename,
  });
  TRes availabilities(
      Iterable<Query$searchAvailability$availabilities?>? Function(
              Iterable<
                  CopyWith$Query$searchAvailability$availabilities<
                      Query$searchAvailability$availabilities>?>?)
          _fn);
}

class _CopyWithImpl$Query$searchAvailability<TRes>
    implements CopyWith$Query$searchAvailability<TRes> {
  _CopyWithImpl$Query$searchAvailability(
    this._instance,
    this._then,
  );

  final Query$searchAvailability _instance;

  final TRes Function(Query$searchAvailability) _then;

  static const _undefined = <dynamic, dynamic>{};

  TRes call({
    Object? availabilities = _undefined,
    Object? $__typename = _undefined,
  }) =>
      _then(Query$searchAvailability(
        availabilities: availabilities == _undefined
            ? _instance.availabilities
            : (availabilities
                as List<Query$searchAvailability$availabilities?>?),
        $__typename: $__typename == _undefined || $__typename == null
            ? _instance.$__typename
            : ($__typename as String),
      ));

  TRes availabilities(
          Iterable<Query$searchAvailability$availabilities?>? Function(
                  Iterable<
                      CopyWith$Query$searchAvailability$availabilities<
                          Query$searchAvailability$availabilities>?>?)
              _fn) =>
      call(
          availabilities: _fn(_instance.availabilities?.map((e) => e == null
              ? null
              : CopyWith$Query$searchAvailability$availabilities(
                  e,
                  (i) => i,
                )))?.toList());
}

class _CopyWithStubImpl$Query$searchAvailability<TRes>
    implements CopyWith$Query$searchAvailability<TRes> {
  _CopyWithStubImpl$Query$searchAvailability(this._res);

  TRes _res;

  call({
    List<Query$searchAvailability$availabilities?>? availabilities,
    String? $__typename,
  }) =>
      _res;

  availabilities(_fn) => _res;
}

const documentNodeQuerysearchAvailability = DocumentNode(definitions: [
  OperationDefinitionNode(
    type: OperationType.query,
    name: NameNode(value: 'searchAvailability'),
    variableDefinitions: [
      VariableDefinitionNode(
        variable: VariableNode(name: NameNode(value: 'gymId')),
        type: NamedTypeNode(
          name: NameNode(value: 'Int'),
          isNonNull: true,
        ),
        defaultValue: DefaultValueNode(value: null),
        directives: [],
      ),
      VariableDefinitionNode(
        variable: VariableNode(name: NameNode(value: 'startTime')),
        type: NamedTypeNode(
          name: NameNode(value: 'String'),
          isNonNull: true,
        ),
        defaultValue: DefaultValueNode(value: null),
        directives: [],
      ),
      VariableDefinitionNode(
        variable: VariableNode(name: NameNode(value: 'endTime')),
        type: NamedTypeNode(
          name: NameNode(value: 'String'),
          isNonNull: true,
        ),
        defaultValue: DefaultValueNode(value: null),
        directives: [],
      ),
      VariableDefinitionNode(
        variable: VariableNode(name: NameNode(value: 'date')),
        type: NamedTypeNode(
          name: NameNode(value: 'String'),
          isNonNull: true,
        ),
        defaultValue: DefaultValueNode(value: null),
        directives: [],
      ),
    ],
    directives: [],
    selectionSet: SelectionSetNode(selections: [
      FieldNode(
        name: NameNode(value: 'availabilities'),
        alias: null,
        arguments: [
          ArgumentNode(
            name: NameNode(value: 'gymId'),
            value: VariableNode(name: NameNode(value: 'gymId')),
          ),
          ArgumentNode(
            name: NameNode(value: 'startTime'),
            value: VariableNode(name: NameNode(value: 'startTime')),
          ),
          ArgumentNode(
            name: NameNode(value: 'endTime'),
            value: VariableNode(name: NameNode(value: 'endTime')),
          ),
          ArgumentNode(
            name: NameNode(value: 'date'),
            value: VariableNode(name: NameNode(value: 'date')),
          ),
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
            name: NameNode(value: 'userId'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
          FieldNode(
            name: NameNode(value: 'gymId'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
          FieldNode(
            name: NameNode(value: 'startTime'),
            alias: null,
            arguments: [],
            directives: [],
            selectionSet: null,
          ),
          FieldNode(
            name: NameNode(value: 'endTime'),
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
Query$searchAvailability _parserFn$Query$searchAvailability(
        Map<String, dynamic> data) =>
    Query$searchAvailability.fromJson(data);
typedef OnQueryComplete$Query$searchAvailability = FutureOr<void> Function(
  Map<String, dynamic>?,
  Query$searchAvailability?,
);

class Options$Query$searchAvailability
    extends graphql.QueryOptions<Query$searchAvailability> {
  Options$Query$searchAvailability({
    String? operationName,
    required Variables$Query$searchAvailability variables,
    graphql.FetchPolicy? fetchPolicy,
    graphql.ErrorPolicy? errorPolicy,
    graphql.CacheRereadPolicy? cacheRereadPolicy,
    Object? optimisticResult,
    Query$searchAvailability? typedOptimisticResult,
    Duration? pollInterval,
    graphql.Context? context,
    OnQueryComplete$Query$searchAvailability? onComplete,
    graphql.OnQueryError? onError,
  })  : onCompleteWithParsed = onComplete,
        super(
          variables: variables.toJson(),
          operationName: operationName,
          fetchPolicy: fetchPolicy,
          errorPolicy: errorPolicy,
          cacheRereadPolicy: cacheRereadPolicy,
          optimisticResult: optimisticResult ?? typedOptimisticResult?.toJson(),
          pollInterval: pollInterval,
          context: context,
          onComplete: onComplete == null
              ? null
              : (data) => onComplete(
                    data,
                    data == null
                        ? null
                        : _parserFn$Query$searchAvailability(data),
                  ),
          onError: onError,
          document: documentNodeQuerysearchAvailability,
          parserFn: _parserFn$Query$searchAvailability,
        );

  final OnQueryComplete$Query$searchAvailability? onCompleteWithParsed;

  @override
  List<Object?> get properties => [
        ...super.onComplete == null
            ? super.properties
            : super.properties.where((property) => property != onComplete),
        onCompleteWithParsed,
      ];
}

class WatchOptions$Query$searchAvailability
    extends graphql.WatchQueryOptions<Query$searchAvailability> {
  WatchOptions$Query$searchAvailability({
    String? operationName,
    required Variables$Query$searchAvailability variables,
    graphql.FetchPolicy? fetchPolicy,
    graphql.ErrorPolicy? errorPolicy,
    graphql.CacheRereadPolicy? cacheRereadPolicy,
    Object? optimisticResult,
    Query$searchAvailability? typedOptimisticResult,
    graphql.Context? context,
    Duration? pollInterval,
    bool? eagerlyFetchResults,
    bool carryForwardDataOnException = true,
    bool fetchResults = false,
  }) : super(
          variables: variables.toJson(),
          operationName: operationName,
          fetchPolicy: fetchPolicy,
          errorPolicy: errorPolicy,
          cacheRereadPolicy: cacheRereadPolicy,
          optimisticResult: optimisticResult ?? typedOptimisticResult?.toJson(),
          context: context,
          document: documentNodeQuerysearchAvailability,
          pollInterval: pollInterval,
          eagerlyFetchResults: eagerlyFetchResults,
          carryForwardDataOnException: carryForwardDataOnException,
          fetchResults: fetchResults,
          parserFn: _parserFn$Query$searchAvailability,
        );
}

class FetchMoreOptions$Query$searchAvailability
    extends graphql.FetchMoreOptions {
  FetchMoreOptions$Query$searchAvailability({
    required graphql.UpdateQuery updateQuery,
    required Variables$Query$searchAvailability variables,
  }) : super(
          updateQuery: updateQuery,
          variables: variables.toJson(),
          document: documentNodeQuerysearchAvailability,
        );
}

extension ClientExtension$Query$searchAvailability on graphql.GraphQLClient {
  Future<graphql.QueryResult<Query$searchAvailability>>
      query$searchAvailability(
              Options$Query$searchAvailability options) async =>
          await this.query(options);
  graphql.ObservableQuery<Query$searchAvailability>
      watchQuery$searchAvailability(
              WatchOptions$Query$searchAvailability options) =>
          this.watchQuery(options);
  void writeQuery$searchAvailability({
    required Query$searchAvailability data,
    required Variables$Query$searchAvailability variables,
    bool broadcast = true,
  }) =>
      this.writeQuery(
        graphql.Request(
          operation:
              graphql.Operation(document: documentNodeQuerysearchAvailability),
          variables: variables.toJson(),
        ),
        data: data.toJson(),
        broadcast: broadcast,
      );
  Query$searchAvailability? readQuery$searchAvailability({
    required Variables$Query$searchAvailability variables,
    bool optimistic = true,
  }) {
    final result = this.readQuery(
      graphql.Request(
        operation:
            graphql.Operation(document: documentNodeQuerysearchAvailability),
        variables: variables.toJson(),
      ),
      optimistic: optimistic,
    );
    return result == null ? null : Query$searchAvailability.fromJson(result);
  }
}

graphql_flutter.QueryHookResult<Query$searchAvailability>
    useQuery$searchAvailability(Options$Query$searchAvailability options) =>
        graphql_flutter.useQuery(options);
graphql.ObservableQuery<Query$searchAvailability>
    useWatchQuery$searchAvailability(
            WatchOptions$Query$searchAvailability options) =>
        graphql_flutter.useWatchQuery(options);

class Query$searchAvailability$Widget
    extends graphql_flutter.Query<Query$searchAvailability> {
  Query$searchAvailability$Widget({
    widgets.Key? key,
    required Options$Query$searchAvailability options,
    required graphql_flutter.QueryBuilder<Query$searchAvailability> builder,
  }) : super(
          key: key,
          options: options,
          builder: builder,
        );
}

class Query$searchAvailability$availabilities {
  Query$searchAvailability$availabilities({
    required this.id,
    required this.userId,
    required this.gymId,
    required this.startTime,
    required this.endTime,
    this.$__typename = 'Availability',
  });

  factory Query$searchAvailability$availabilities.fromJson(
      Map<String, dynamic> json) {
    final l$id = json['id'];
    final l$userId = json['userId'];
    final l$gymId = json['gymId'];
    final l$startTime = json['startTime'];
    final l$endTime = json['endTime'];
    final l$$__typename = json['__typename'];
    return Query$searchAvailability$availabilities(
      id: (l$id as int),
      userId: (l$userId as int),
      gymId: (l$gymId as int),
      startTime: (l$startTime as String),
      endTime: (l$endTime as String),
      $__typename: (l$$__typename as String),
    );
  }

  final int id;

  final int userId;

  final int gymId;

  final String startTime;

  final String endTime;

  final String $__typename;

  Map<String, dynamic> toJson() {
    final _resultData = <String, dynamic>{};
    final l$id = id;
    _resultData['id'] = l$id;
    final l$userId = userId;
    _resultData['userId'] = l$userId;
    final l$gymId = gymId;
    _resultData['gymId'] = l$gymId;
    final l$startTime = startTime;
    _resultData['startTime'] = l$startTime;
    final l$endTime = endTime;
    _resultData['endTime'] = l$endTime;
    final l$$__typename = $__typename;
    _resultData['__typename'] = l$$__typename;
    return _resultData;
  }

  @override
  int get hashCode {
    final l$id = id;
    final l$userId = userId;
    final l$gymId = gymId;
    final l$startTime = startTime;
    final l$endTime = endTime;
    final l$$__typename = $__typename;
    return Object.hashAll([
      l$id,
      l$userId,
      l$gymId,
      l$startTime,
      l$endTime,
      l$$__typename,
    ]);
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (!(other is Query$searchAvailability$availabilities) ||
        runtimeType != other.runtimeType) {
      return false;
    }
    final l$id = id;
    final lOther$id = other.id;
    if (l$id != lOther$id) {
      return false;
    }
    final l$userId = userId;
    final lOther$userId = other.userId;
    if (l$userId != lOther$userId) {
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
    final l$$__typename = $__typename;
    final lOther$$__typename = other.$__typename;
    if (l$$__typename != lOther$$__typename) {
      return false;
    }
    return true;
  }
}

extension UtilityExtension$Query$searchAvailability$availabilities
    on Query$searchAvailability$availabilities {
  CopyWith$Query$searchAvailability$availabilities<
          Query$searchAvailability$availabilities>
      get copyWith => CopyWith$Query$searchAvailability$availabilities(
            this,
            (i) => i,
          );
}

abstract class CopyWith$Query$searchAvailability$availabilities<TRes> {
  factory CopyWith$Query$searchAvailability$availabilities(
    Query$searchAvailability$availabilities instance,
    TRes Function(Query$searchAvailability$availabilities) then,
  ) = _CopyWithImpl$Query$searchAvailability$availabilities;

  factory CopyWith$Query$searchAvailability$availabilities.stub(TRes res) =
      _CopyWithStubImpl$Query$searchAvailability$availabilities;

  TRes call({
    int? id,
    int? userId,
    int? gymId,
    String? startTime,
    String? endTime,
    String? $__typename,
  });
}

class _CopyWithImpl$Query$searchAvailability$availabilities<TRes>
    implements CopyWith$Query$searchAvailability$availabilities<TRes> {
  _CopyWithImpl$Query$searchAvailability$availabilities(
    this._instance,
    this._then,
  );

  final Query$searchAvailability$availabilities _instance;

  final TRes Function(Query$searchAvailability$availabilities) _then;

  static const _undefined = <dynamic, dynamic>{};

  TRes call({
    Object? id = _undefined,
    Object? userId = _undefined,
    Object? gymId = _undefined,
    Object? startTime = _undefined,
    Object? endTime = _undefined,
    Object? $__typename = _undefined,
  }) =>
      _then(Query$searchAvailability$availabilities(
        id: id == _undefined || id == null ? _instance.id : (id as int),
        userId: userId == _undefined || userId == null
            ? _instance.userId
            : (userId as int),
        gymId: gymId == _undefined || gymId == null
            ? _instance.gymId
            : (gymId as int),
        startTime: startTime == _undefined || startTime == null
            ? _instance.startTime
            : (startTime as String),
        endTime: endTime == _undefined || endTime == null
            ? _instance.endTime
            : (endTime as String),
        $__typename: $__typename == _undefined || $__typename == null
            ? _instance.$__typename
            : ($__typename as String),
      ));
}

class _CopyWithStubImpl$Query$searchAvailability$availabilities<TRes>
    implements CopyWith$Query$searchAvailability$availabilities<TRes> {
  _CopyWithStubImpl$Query$searchAvailability$availabilities(this._res);

  TRes _res;

  call({
    int? id,
    int? userId,
    int? gymId,
    String? startTime,
    String? endTime,
    String? $__typename,
  }) =>
      _res;
}
