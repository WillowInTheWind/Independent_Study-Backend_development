import { Component } from '@angular/core';
import {AuthorizationService} from "../authorization.service";
import {RouterOutlet} from "@angular/router";
import {CookieService} from "ngx-cookie-service";
import {FormBuilder, FormControl, FormGroup, ReactiveFormsModule} from "@angular/forms";
import {MorningExService} from "../morning-ex.service";

@Component({
  selector: 'app-homepage',
  standalone: true,
  imports: [
    RouterOutlet,
    ReactiveFormsModule
  ],
  templateUrl: './homepage.component.html',
  styleUrl: './homepage.component.css'
})
export class HomepageComponent {
  submitMX() {
    if (!this.mxform.value.title || !this.mxform.value.date ) {
      return
    }
    this.mxManager.postMx(
      <string>this.mxform.value.date,
      <string>this.mxform.value.title,
      <string>this.mxform.value.description,
    )
    this.mxform.reset()
  }

  mxform = new FormGroup({
    date:  new FormControl('', {nonNullable: true}),
    title: new FormControl('', {nonNullable: true}),
    description: new FormControl('', {nonNullable: true}),
  })
  login( ) {
    this.hello.OauthLogin();
  }

  test( ) {
    this.hello.test()
  }

  cookie() {
    console.log(this.cookies.get("token"))
  }
  constructor(private hello: AuthorizationService, private cookies: CookieService, private mxManager: MorningExService) {
    // this.login()
  }
}

