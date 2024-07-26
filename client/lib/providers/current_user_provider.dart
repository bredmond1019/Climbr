import 'package:client/models/user.dart';
import 'package:flutter/material.dart';

class CurrentUserProvider with ChangeNotifier {
  // CurrentUser? _user;
  CurrentUser _user =
      CurrentUser(id: 1, name: 'John Doe', email: 'test@gmail.com');

  CurrentUser get user => _user;

  void setUser(CurrentUser user) {
    _user = user;
    notifyListeners();
  }
}
