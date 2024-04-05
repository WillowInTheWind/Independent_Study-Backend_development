import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import {AuthorizationService} from "../authorization.service";
import {HTTP_INTERCEPTORS, HttpClientModule} from "@angular/common/http";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, HttpClientModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'Mx-Management-Site';
  login( ) {
    this.hello.OauthLogin();
  }

  constructor(private hello: AuthorizationService) {
    // this.login()
  }
}
