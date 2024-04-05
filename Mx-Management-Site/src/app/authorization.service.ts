import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root',
})
export class AuthorizationService {
  OauthLogin() {
    let me = this.http.get<string>("http://127.0.0.1:8080/auth/login").subscribe(data => {})
    console.log(me)
  }
  constructor(public http: HttpClient) { }
}
