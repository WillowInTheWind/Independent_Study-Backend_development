import { Component } from '@angular/core';
import { RouterOutlet} from '@angular/router';
import { HttpClientModule} from "@angular/common/http";

import {MatToolbar, MatToolbarModule} from '@angular/material/toolbar';
import { MatIconModule} from '@angular/material/icon';
import { CookieService } from 'ngx-cookie-service';
import { OnInit} from "@angular/core";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [HttpClientModule, RouterOutlet, MatIconModule, MatToolbar, MatToolbarModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'Mx-Management-Site';

  constructor(private cookieService: CookieService) {
  }
  ngOnInit() {
    }
}
