import { Component } from '@angular/core';
import {MatMenuItem} from "@angular/material/menu";
import {NgIf} from "@angular/common";
import {AuthorizationService} from "../authorization.service";
import {MatDialog} from "@angular/material/dialog";
import {CookieService} from "ngx-cookie-service";

@Component({
  selector: 'app-error-page',
  standalone: true,
    imports: [
        MatMenuItem,
        NgIf
    ],
  templateUrl: './error-page.component.html',
  styleUrl: './error-page.component.css'
})
export class ErrorPageComponent {

  login( ) {
    this.user.OauthLogin();
  }

  constructor( private user: AuthorizationService) {
  }
}
