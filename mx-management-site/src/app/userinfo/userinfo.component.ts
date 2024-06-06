import { Component } from '@angular/core';
import {Router} from "@angular/router";
import {AuthorizationService} from "../authorization.service";
import {GoogleUser} from "../authorization.service";
import {AsyncPipe, NgForOf} from "@angular/common";
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {Observable} from "rxjs";
import {
    MatExpansionPanel,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MxinfoComponent} from "../mxinfo/mxinfo.component";
import {MatDialog} from "@angular/material/dialog";
import {MxinfouserComponent} from "../mxinfouser/mxinfouser.component";
@Component({
  selector: 'app-userinfo',
  standalone: true,
    imports: [
        AsyncPipe,
        NgForOf,
        MatExpansionPanel,
        MatExpansionPanelDescription,
        MatExpansionPanelHeader,
        MatExpansionPanelTitle
    ],
  templateUrl: './userinfo.component.html',
  styleUrl: './userinfo.component.css'
})
export class UserinfoComponent {
  viewMX (title: string) {
    var morningExercise: Observable<MorningExercise[]> = this.mx.getmx(title);
    const dialogRef = this.dialog.open(MxinfouserComponent, {
        width: '90vw',
        data: {
          mx: morningExercise,
          iseditor: false
        }
      }
    );
  }
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
    private route: Router, private auth: AuthorizationService, private mx: MorningExService,
    private dialog: MatDialog
  ) {

  }
}
