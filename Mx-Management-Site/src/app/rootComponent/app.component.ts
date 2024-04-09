import { Component } from '@angular/core';
import { RouterOutlet} from '@angular/router';
import { HttpClientModule} from "@angular/common/http";

import {MatToolbar, MatToolbarModule} from '@angular/material/toolbar';
import { MatIconModule} from '@angular/material/icon';
import { CookieService } from 'ngx-cookie-service';
import { OnInit} from "@angular/core";
import {AuthorizationService} from "../authorization.service";
import {CommonModule, NgIf} from "@angular/common";
import {MatMenu, MatMenuItem, MatMenuTrigger} from "@angular/material/menu";


@Component({
  selector: 'app-root',
  standalone: true,
  imports: [HttpClientModule, RouterOutlet, MatIconModule, MatToolbar, MatToolbarModule, CommonModule, MatMenu, MatMenuTrigger, MatMenuItem],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'Mx-Management-Site';

  protected image = "";
  username = "";
  login( ) {
    this.user.OauthLogin();
  }

  isloggedin( ) {
    if (this.cookie.get("token")) {
      return true
    }
    else {
      return false
    }
  }
  logout() {
    this.cookie.delete("token")
  }

  constructor(private user: AuthorizationService, private  cookie: CookieService) {
  }
  ngOnInit() {
    this.user.getuser().then(r => r.subscribe( date => {
      this.image = date.picture;
      this.username = date.name;
    }))
    }

}
