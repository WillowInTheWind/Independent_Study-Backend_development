import {Inject, Injectable} from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { DOCUMENT } from '@angular/common';

@Injectable({
  providedIn: 'root',
})
export class AuthorizationService {
  OauthLogin() {
    let me = this.http.get<string>("http://127.0.0.1:8080/auth/login").subscribe(data => {
      this.document.location.href = data
    })
    console.log(me)
  }
  constructor(public http: HttpClient, @Inject(DOCUMENT) private document: Document) { }
}
