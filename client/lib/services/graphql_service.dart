import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class GraphQLService {
  static final HttpLink httpLink = HttpLink(
    'http://localhost:8080/graphql',
  );

  // static final AuthLink authLink = AuthLink(
  //   getToken: () async => 'Bearer <YOUR_API_KEY>',
  // );

  // static final Link link = authLink.concat(httpLink);

  static ValueNotifier<GraphQLClient> initailizeClient() {
    ValueNotifier<GraphQLClient> client = ValueNotifier(
      GraphQLClient(
        cache: GraphQLCache(store: InMemoryStore()),
        link: httpLink,
      ),
    );

    return client;
  }
}
