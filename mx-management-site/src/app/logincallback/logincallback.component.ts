import {Component, Inject} from '@angular/core';
import {ActivatedRoute, Router, RouterLink, RouterModule, RouterOutlet} from "@angular/router";
import {OnInit} from "@angular/core";
import {AuthorizationService} from "../authorization.service";
import {HttpClientModule} from "@angular/common/http";
import {DOCUMENT} from "@angular/common";
import {CookieService} from "ngx-cookie-service";

@Component({
  selector: 'app-logincallback',
  standalone: true,
  imports: [RouterOutlet,RouterLink, HttpClientModule ],
  templateUrl: './logincallback.component.html',
  styleUrl: './logincallback.component.css'
})
export class LogincallbackComponent {

  queryparams: string = "";
  user = "";
  constructor(private route: Router,
              private authservice: AuthorizationService,
              @Inject(DOCUMENT) private document: Document,
              private cookies: CookieService)
  {
  }


  async ngOnInit() {
    let url = this.route.url;
    this.queryparams = url.replace("/auth", "")
    if (this.queryparams.slice(0,6) != "?state") {
      this.route.navigate(['']);
    }
    else {
      this.queryparams = this.queryparams.replace("?", "")
      await this.authservice.sendCode(this.queryparams);
    }
  }
}
