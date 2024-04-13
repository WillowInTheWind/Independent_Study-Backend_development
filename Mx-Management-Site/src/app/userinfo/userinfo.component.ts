import { Component } from '@angular/core';
import {Router} from "@angular/router";
import {AuthorizationService} from "../authorization.service";
import {GoogleUser} from "../authorization.service";
import {AsyncPipe, NgForOf} from "@angular/common";
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {Observable} from "rxjs";
@Component({
  selector: 'app-userinfo',
  standalone: true,
  imports: [
    AsyncPipe,
    NgForOf
  ],
  templateUrl: './userinfo.component.html',
  styleUrl: './userinfo.component.css'
})
export class UserinfoComponent {

  picture: string  = '' ;
  email: string = ' ';
  name: string = '';
  phone_number: string = '';

  morningExs : Observable<MorningExercise[]> = new Observable<MorningExercise[]>();

  ngOnInit() {
    this.auth.getuserbyname(this.route.url).then(r => {
      r.subscribe(
        data => {
          this.name= data.name;
          this.email= data.email;

          this.picture= data.picture;
          this.phone_number= data.phone_number;
          this.morningExs = this.mx.getusermxsbyname(this.name)

        } )
    }
    )
  }

  constructor(
    private route: Router, private auth: AuthorizationService, private mx: MorningExService
  ) {

  }
}
