import { Component } from '@angular/core';
import {Observable} from "rxjs";
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {AsyncPipe, CommonModule} from "@angular/common";
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
  selector: 'app-user-mxs',
  standalone: true,
  imports: [
    AsyncPipe, CommonModule, MatExpansionPanel, MatExpansionPanelDescription, MatExpansionPanelHeader, MatExpansionPanelTitle
  ],
  templateUrl: './user-mxs.component.html',
  styleUrl: './user-mxs.component.css'
})
export class UserMxsComponent {
  protected morningExs: Observable<MorningExercise[]> = this.mx.getusermxs();

  viewMX (title: string) {
    var morningExercise: Observable<MorningExercise[]> = this.mx.getmx(title);
    const dialogRef = this.dialog.open(MxinfouserComponent, {
      width: '90vw',
      data: {
        mx: morningExercise,
        iseditor: true
      },
    });
  }
  editMx(title: string) {
    // this.mx.editMx(title)
  }
  constructor(
    private mx: MorningExService,
    private dialog: MatDialog
  ) {
  }
}
