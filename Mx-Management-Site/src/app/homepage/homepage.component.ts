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
    this.mxManager.postMx(
      parseInt(<string>this.mxform.value.mx_index),
      <string>this.mxform.value.date,
      parseInt(<string>this.mxform.value.owner),
      <string>this.mxform.value.title,
      <string>this.mxform.value.description,
    )
    this.mxform.reset()
  }

  mxform = new FormGroup({
    mx_index: new FormControl('', {nonNullable: true}),
    date:  new FormControl('', {nonNullable: true}),
    owner: new FormControl('', {nonNullable: true}) ,
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

