import { Component } from '@angular/core';
import {RouterOutlet} from "@angular/router";
import {MxFormComponent} from "../mx-form/mx-form.component";
import {MatCalendar} from "@angular/material/datepicker";
import {AsyncPipe, CommonModule, NgForOf, NgIf} from "@angular/common";
import {Observable, of} from "rxjs";
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {MxinfoComponent} from "../mxinfo/mxinfo.component";
import {MatDialog} from "@angular/material/dialog";
import {AuthorizationService} from "../authorization.service";
import {MxinfouserComponent} from "../mxinfouser/mxinfouser.component";
// import {MxFormComponent} from "../mx-form/mx-form.component";
// import {MatCalendar} from "@angular/material/datepicker";

@Component({
  selector: 'app-homepage',
  standalone: true,
    imports: [
        RouterOutlet,
        MxFormComponent,
        MatCalendar,
        AsyncPipe,
        NgForOf,
        NgIf,
      CommonModule,
    ],
  templateUrl: './homepage.component.html',
  styleUrl: './homepage.component.css'
})
export class HomepageComponent {
  protected MorningExs: Observable<MorningExercise[]> = this.mx.getupcomingmxs();
  viewMX (morningexercise: MorningExercise) {
    const dialogRef = this.dialog.open(MxinfouserComponent, {
      width: '90vw',
      data: {
        mx: of(morningexercise),
        iseditor: false,
        isdisplay: true
      }
    });
  }

  constructor(
    private mx: MorningExService,
    private dialog: MatDialog,
    private auth: AuthorizationService,
  ) {

  }

}
