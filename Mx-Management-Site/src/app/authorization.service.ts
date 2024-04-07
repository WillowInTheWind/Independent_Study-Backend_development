import {Inject, Injectable} from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { DOCUMENT } from '@angular/common';
import {CookieService} from "ngx-cookie-service";

@Injectable({
  providedIn: 'root',
})
export class AuthorizationService {
  private jwttoken: string = " ";
  test() {
    this.http.get("/api", {
      withCredentials: true}).subscribe(data => {
    })
  }
  OauthLogin() {
    let me = this.http.get<string>("/api/auth/login").subscribe(data => {
      console.log(data)
      this.document.location.href = data
    })
    console.log(me)
  }
   async sendCode(code: string) {
    const url: string = "/api/auth/authorized?" + code;
    let data = this.http.get<string>(url, {observe: 'response'})
      .subscribe(data => {
    }
    )
  }
  constructor(public http: HttpClient, @Inject(DOCUMENT) private document: Document, private cookies:  CookieService) { }
}
